//! # Oracle
//!
//! `oracle` is a module for oracle, providing support for oracle registration and revoking etc
//! This can be compiled with `#[no_std]`, ready for Wasm
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use rstd::prelude::*;
use rstd::result;

use sr_primitives::traits::{
    Bounded, CheckedAdd, CheckedSub, EnsureOrigin, Hash, Saturating, Zero,
};
use support::traits::{
    ChangeMembers, Currency, Get, LockIdentifier, LockableCurrency, ReservableCurrency,
    WithdrawReason, WithdrawReasons,
};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue,
};
use system::{ensure_root, ensure_signed};

#[cfg(test)]
mod oracle_test;

/// The status of oracle
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq)]
pub enum OracleStatus {
    None,
    /// The oracle is active
    Active,
    /// The oracle is offline
    Offline,
    /// The oracle is forbidden
    Forbidden,
}

impl Default for OracleStatus {
    fn default() -> Self {
        OracleStatus::None
    }
}

/// The info struct for statistic information of oracle
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct OracleInfo<Balance> {
    /// Total jobs requested
    pub total_jobs: u64,
    /// Total missed jobs
    pub total_missed_jobs: u64,
    /// Total witnessed jobs
    pub total_witnessed_jobs: u64,
    /// Total reward received
    pub total_reward: Balance,
    /// Withdrawable funds
    pub withdrawable_reward: Balance,
    /// Total slashed funds
    pub total_slash: Balance,
    /// Oracle status TODO:
    pub status: OracleStatus,
}

/// The job struct
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Job<BlockNumber, Balance, AccountId> {
    /// The requestor of the job
    pub from: AccountId,
    /// The metadata for this job, including business type etc
    /// TODO: convert to serde json
    pub meta: Vec<u8>,
    /// The height of blockchain when job is created
    pub created_at: BlockNumber,
    /// The height of blockchain after which job is timeout
    pub expired_at: BlockNumber,
    /// Oracle requested
    pub oracle: AccountId,
    /// Reward for this job
    pub reward: Balance,
    /// Nocne value
    pub nonce: u64,
}

pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
// type PositiveImbalanceOf<T> =
// 	<<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
// type NegativeImbalanceOf<T> =
// 	<<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

type LedgerOf<T> = Ledger<BalanceOf<T>, <T as system::Trait>::BlockNumber>;
type JobOf<T> = Job<
    <T as system::Trait>::BlockNumber,
    BalanceOf<T>,
    <T as system::Trait>::AccountId,
>;

const LOCKED_ID: LockIdentifier = *b"oracle  ";

pub trait Trait: balances::Trait {
    /// The maximum delay value for job execution
    type MaxTimeout: Get<Self::BlockNumber>;
    /// The amount of fee that should be paid to each oracle during each reporting cycle.
    type OracleFee: Get<BalanceOf<Self>>;
    /// The amount that'll be slashed if one oracle missed its reporting window.
    type MissReportSlash: Get<BalanceOf<Self>>;
    /// The minimum amount to stake for an oracle candidate.
    type MinStaking: Get<BalanceOf<Self>>;
    /// The origin that's responsible for slashing malicious oracles.
    // type MaliciousSlashOrigin: EnsureOrigin<Self::Origin>;
    /// The maxium count of working oracles.
    type Count: Get<u16>;
    /// The duration in which oracles should report and be paid.
    type ReportInteval: Get<Self::BlockNumber>;
    /// The duration between oracle elections.
    type ElectionEra: Get<Self::BlockNumber>;
    /// The locked time of staked amount.
    type LockedDuration: Get<Self::BlockNumber>;
    /// The actual oracle membership management type. (Usually the `srml_collective::Trait`)
    type ChangeMembers: ChangeMembers<Self::AccountId>;
    /// Event type
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// Currency type
    type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>
        + ReservableCurrency<Self::AccountId>;
}

/// Unbond record of an oracle/candidate
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Unbond<Balance, BlockNumber> {
    amount: Balance,
    until: BlockNumber,
}

/// The ledger of oracle's staked token.
#[derive(PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Ledger<Balance: Default, BlockNumber> {
    /// Total locked funds
    locked: Balance,
    /// Total staked funds
    staked: Balance,
    /// Pending unbonds
    unbonds: Vec<Unbond<Balance, BlockNumber>>,
}

impl<Balance: Default, BlockNumber> Default for Ledger<Balance, BlockNumber> {
    fn default() -> Self {
        Ledger {
            locked: Balance::default(),
            staked: Balance::default(),
            unbonds: vec![],
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        Hash = <T as system::Trait>::Hash,
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        BlockNumber = <T as system::Trait>::BlockNumber,
    {
        /// Amount bonded by one oracle.
        OracleBonded(AccountId, Balance),
        /// Amount unbonded by one oracle.
        OracleUnbonded(AccountId, Balance),
        /// Oracle is removed
        OracleRemoved(AccountId),
        /// Amount slashed to one oracle.
        OracleSlashed(AccountId, Balance),
        /// Amount paid to one oracle.
        OraclePaid(AccountId, Balance),
        /// Candidate added.
        CandidateAdded(AccountId),
        /// Candidate remove.
        CandidateRemoved(AccountId),
        /// Unqualified member added
        UnqualifiedMemberAdded(AccountId),
        /// Unqualified member removed
        UnqualifiedMemberRemoved(AccountId),
        /// Amount unlocked for one oracle.
        OracleStakeReleased(AccountId, Balance),
        /// Job created
        JobCreated(AccountId, AccountId, BlockNumber, Hash),
        /// Job cancelled
        JobCancelled(AccountId, BlockNumber, Hash),
        /// Job fulfilled
        JobFulfilled(AccountId, BlockNumber, Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as OracleStorage {
        /// Acting oracle accounts.
        Oracles get(oracles): Vec<T::AccountId>;

        /// Oracle statistic information
        OracleInfos get(oracle_info): map T::AccountId => OracleInfo<BalanceOf<T>>;

        /// Staking ledgers of oracle/candidates.
        Ledgers get(ledger): map T::AccountId => LedgerOf<T>;

        /// Job map
        Jobs get(job): map T::Hash => JobOf<T>;

        /// Blockstamp of each oracle's last event report.
        WitnessReport get(witness_report): map T::AccountId => T::BlockNumber;

        /// Candidates of oracles
        Candidates get(candidates): Vec<T::AccountId>;

        /// Unqualified members
        UnqualifiedMembers get(unqualified_members): Vec<T::AccountId>;

        /// Current election era.
        CurrentEra get(current_era): T::BlockNumber;

        /// Oracle reward records.
        LastRewardedOracles get(last_rewarded_oracles): map T::AccountId => T::BlockNumber;

        /// The cashier account
        CashierAccount get(cashier_account) config(): T::AccountId;

        /// The nonce value for hash of job
        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        const MaxTimeout: T::BlockNumber = T::MaxTimeout::get();
        const OracleFee: BalanceOf<T> = T::OracleFee::get();
        const MissReportSlash: BalanceOf<T> = T::MissReportSlash::get();
        const MinStaking: BalanceOf<T> = T::MinStaking::get();
        const Count: u16 = T::Count::get();
        const ElectionEra: T::BlockNumber = T::ElectionEra::get();
        const ReportInteval: T::BlockNumber = T::ReportInteval::get();
        const LockedDuration: T::BlockNumber = T::LockedDuration::get();

        /// bond amount to list as oraclce candidates.
        ///
        /// @origin
        /// @amount the amount to be bound
        pub fn bond(origin, amount: BalanceOf<T>) -> Result{
            let sender = ensure_signed(origin)?;

            Self::do_bond(&sender, amount)?;
            let _ = Self::add_candidate(&sender);
            Ok(())
        }

        /// slash oracle by governors
        ///
        /// @origin     the initiator
        /// @who        the oracle to be slashed
        /// @amount     the amount of slashing
        // pub fn slash_by_vote(origin, who: T::AccountId, amount: BalanceOf<T>) -> Result{
        //     T::MaliciousSlashOrigin::try_origin(origin)
        //         .map(|_| ())
        //         .or_else(ensure_root)
        //         .map_err(|_| "bad origin")?;
        //     T::Currency::slash(&who, amount);
        //     Self::deposit_event(RawEvent::OracleSlashed(who, amount));
        //     Ok(())
        // }

        /// Unbond amount
        ///
        /// @origin     the sender
        /// @amount     the amount of funds to be unbond
        pub fn unbond(origin, amount: BalanceOf<T>) -> Result{
            let sender = ensure_signed(origin)?;
            Self::do_unbond(&sender, amount)
        }

        /// Claim rewards
        ///
        /// @origin the sender
        /// @amount the amount to be withdrawed
        pub fn claim_reward(origin, amount: BalanceOf<T>) -> Result {
            ensure!(!amount.is_zero(), "Amount should not be zero");
            let oracle = ensure_signed(origin)?;
            let mut info = Self::oracle_info(oracle.clone());
            ensure!(amount <= info.withdrawable_reward, "Exceed withdrawable funds");
            info.withdrawable_reward = info.withdrawable_reward.saturating_sub(amount);

            T::Currency::transfer(&Self::cashier_account(), &oracle, amount)?;
            <OracleInfos<T>>::insert(oracle.clone(), info);

            Ok(())
        }

        /// Actions when finalizing a block:
        ///     1. Start an election at the right moment.
        ///     2. Release due locked stake.
        ///
        /// @block_number   current block number
        fn on_finalize(block_number: T::BlockNumber) {
            // let current_era = Self::current_era();
            // if block_number == current_era + T::ElectionEra::get() {
            //     Self::elect_oracles();
            //     <CurrentEra<T>>::put(block_number+T::ElectionEra::get());
            // }
            if T::BlockNumber::zero() == block_number % T::ElectionEra::get() {
                Self::elect_oracles();
            }
            Self::release_due_locked_funds(block_number);
            Self::cleanup();
        }

    }
}

/// Helper functions
impl<T: Trait> Module<T> {
    /// Elect oracles
    /// We choose the top N candidates according to staked funds
    fn elect_oracles() {
        let current_oracles = Self::oracles();
        let current_candidates = Self::candidates();
        let mut all_accounts: Vec<T::AccountId> = Vec::new();

        all_accounts.extend(current_candidates);
        all_accounts.extend(current_oracles.clone());

        if all_accounts.len() < T::Count::get().into() {
            // TODO: reset oracles?
            return;
        }

        let mut all_ledgers: Vec<(&T::AccountId, LedgerOf<T>)> = all_accounts
            .iter()
            .map(|a| {
                let ledger = Self::ledger(a);
                (a, ledger)
            })
            .collect();

        all_ledgers.sort_by_key(|(_, ledger)| ledger.staked);
        // Convert to descending order
        all_ledgers.reverse();

        let all_candidates = all_ledgers
            .into_iter()
            .map(|(a, _)| a.clone())
            .collect::<Vec<T::AccountId>>();
        let (chosen_oracles, remaining_candidates) =
            all_candidates.split_at(T::Count::get().into());

        let mut chosen_oracles = chosen_oracles.to_vec();
        // FIXME: is it necessary?
        // chosen_oracles.sort();
        let new_oracles: Vec<T::AccountId> = chosen_oracles
            .clone()
            .into_iter()
            .filter(|o| !current_oracles.contains(&o))
            .collect();

        let outgoing_oracles: Vec<T::AccountId> = current_oracles
            .into_iter()
            .filter(|o| !new_oracles.contains(&o))
            .collect();

        <Oracles<T>>::put(&chosen_oracles);
        <Candidates<T>>::put(remaining_candidates.to_vec());
        T::ChangeMembers::change_members(&new_oracles, &outgoing_oracles, chosen_oracles);
    }

    /// Release due locked funds
    fn release_due_locked_funds(current_height: T::BlockNumber) {
        let oracles = Self::oracles();
        let candidates = Self::candidates();
        let members = Self::unqualified_members();

        oracles
            .iter()
            .chain(candidates.iter())
            .chain(members.iter())
            .for_each(|who| Self::remove_expired_lock(who, current_height));
    }

    /// Remove expired locks
    ///
    /// @who    the owner account
    /// @current_height the height of chain
    fn remove_expired_lock(who: &T::AccountId, current_height: T::BlockNumber) {
        let mut ledger = Self::ledger(who);
        let mut released_funds = <BalanceOf<T>>::zero();

        ledger.unbonds = ledger
            .unbonds
            .into_iter()
            .filter(|x| {
                if x.until >= current_height {
                    released_funds = released_funds.saturating_add(x.amount);
                    false
                } else {
                    true
                }
            })
            .collect();

        if released_funds.is_zero() {
            return;
        }

        ledger.locked = ledger.locked.saturating_sub(released_funds);
        T::Currency::set_lock(
            LOCKED_ID,
            who,
            ledger.locked,
            T::BlockNumber::max_value(),
            WithdrawReasons::all(),
        );
        <Ledgers<T>>::insert(who, ledger);

        Self::deposit_event(RawEvent::OracleStakeReleased(who.clone(), released_funds));
    }

    /// Cleanup dust unqualified members
    fn cleanup() {
        let mut members = Self::unqualified_members();
        let mut count: u64 = 0;

        members.retain(|m| {
            let ledger = Self::ledger(m.clone());
            if ledger.locked.is_zero() {
                count += 1;
                T::Currency::remove_lock(LOCKED_ID, m);
                return false;
            }
            // Unlock the remaining small funds
            // if !ledger.staked.is_zero() {
            //     let _ = Self::do_unbond(m, ledger.staked);
            // }

            true
        });

        if count > 0 {
            <UnqualifiedMembers<T>>::put(members);
        }
    }
}

impl<T: Trait> Module<T> {
    /// Bond some token
    ///
    /// @who the candidate
    /// @amount the amount of token
    fn do_bond(who: &T::AccountId, amount: BalanceOf<T>) -> Result {
        ensure!(!amount.is_zero(), "Amount should not be zero");
        let current_balance = T::Currency::free_balance(who);
        let new_balance = current_balance
            .checked_sub(&amount)
            .ok_or("Not enough money")?;
        // FIXME: count on transfering fee?
        // let fee = T::TransferFee::get();
		// let liability = match amount.checked_add(&fee) {
		// 	Some(l) => l,
		// 	None => return Err("got overflow after adding a fee to value"),
		// };

		// let new_balance = match current_balance.checked_sub(&liability) {
		// 	None => return Err("balance too low to send value"),
		// 	Some(b) => b,
		// };

        ensure!(
            Ok(()) ==
            T::Currency::ensure_can_withdraw(who, amount, WithdrawReason::Transfer, new_balance),
            "Cannot stake more funds than owned"
        );
        
        // If it's a new user, a default ledger will be returned
        let mut ledger = Self::ledger(who);
        let new_locked = ledger
            .locked
            .checked_add(&amount)
            .ok_or("Error calculating new locked funds")?;

        let new_staked = ledger
            .staked
            .checked_add(&amount)
            .ok_or("Error calculating new staked funds")?;

        ensure!(
            new_staked >= T::MinStaking::get(),
            "Total staked amount is too small"
        );
        // Update ledger of this account
        ledger.locked = new_locked;
        ledger.staked = new_staked;
        <Ledgers<T>>::insert(who, ledger);
        // FIXME: Lock amount of token?
        T::Currency::set_lock(
            LOCKED_ID,
            &who,
            new_locked,
            T::BlockNumber::max_value(),
            WithdrawReasons::all(),
        );
        Self::deposit_event(RawEvent::OracleBonded(who.clone(), amount));
        Ok(())
    }

    /// Add a new candidate
    ///
    /// @who    the candidate account
    fn add_candidate(who: &T::AccountId) -> bool {
        if Self::oracles().contains(&who) {
            return false;
        }
        let mut candidates = Self::candidates();
        if candidates.contains(&who) {
            return false;
        }
        candidates.push(who.clone());
        <Candidates<T>>::put(candidates);

        Self::deposit_event(RawEvent::CandidateAdded(who.clone()));
        Self::remove_unqualified_member(who);
        true
    }
    /// Remove a candidate
    ///
    /// @who    the account to be removed from candidate list
    fn remove_candidate(who: &T::AccountId) -> bool {
        let mut candidates = Self::candidates();
        if !candidates.contains(who) {
            return false;
        }

        candidates.retain(|o| o != who);
        <Candidates<T>>::put(&candidates);

        Self::deposit_event(RawEvent::CandidateRemoved(who.clone()));
        true
    }

    /// Remove oracle
    ///
    /// @who    the account to be removed from oracle list
    fn remove_oracle(who: &T::AccountId) -> bool {
        let mut oracles = Self::oracles();
        if !oracles.contains(who) {
            return false;
        }

        oracles.retain(|o| o != who);
        <Oracles<T>>::put(&oracles);

        Self::deposit_event(RawEvent::OracleRemoved(who.clone()));
        true
    }

    /// Add unqualified member
    ///
    /// @who    the account to be added to unqualified list
    fn add_unqualified_member(who: &T::AccountId) -> bool {
        let mut members = Self::unqualified_members();
        if members.contains(who) {
            return false;
        }

        members.push(who.clone());
        <UnqualifiedMembers<T>>::put(members);
        Self::deposit_event(RawEvent::UnqualifiedMemberAdded(who.clone()));
        true
    }

    /// Remove unqualified member
    ///
    /// @who    the account to be removed from unqualified list
    fn remove_unqualified_member(who: &T::AccountId) -> bool {
        let mut members = Self::unqualified_members();
        if !members.contains(who) {
            return false;
        }

        members.retain(|m| m != who);
        <UnqualifiedMembers<T>>::put(members);
        Self::deposit_event(RawEvent::UnqualifiedMemberRemoved(who.clone()));
        true
    }

    /// Slash fund of an oracle
    ///
    /// @who the account whose funds are to be slashed
    /// @amount the amount of funds
    // fn do_slash(who: &T::AccountId, amount: BalanceOf<T>) -> Result {
    //     let mut ledger = Self::ledger(who);

    //     let slash_amount = if amount > ledger.staked {
    //         // Remove this oracle
    //         <Oracles<T>>::mutate(|v| v.retain(|item| item != who));

    //         let current_oracles = Self::oracles();
    //         T::ChangeMembers::change_members(&[], &[who.clone()], current_oracles);
    //         ledger.staked
    //     } else {
    //         amount
    //     };

    //     // TODO: Handle imbalance
    //     T::Currency::slash(who, amount);
    //     ledger.staked = ledger
    //         .staked
    //         .checked_sub(&slash_amount)
    //         .ok_or("Error calculating new staking")?;
    //     <Ledgers<T>>::insert(who, ledger);

    //     Self::deposit_event(RawEvent::OracleSlashed(who.clone(), slash_amount));
    //     Ok(())
    // }

    /// Unbond the funds previous bonded
    /// TODO:
    /// @who the account
    /// @amount the amount of funds
    fn do_unbond(who: &T::AccountId, amount: BalanceOf<T>) -> Result {
        let current_height = Self::block_number();
        let mut ledger = Self::ledger(who);

        ensure!(
            !amount.is_zero() && amount <= ledger.staked,
            "The unbond amount is zero or larger than staked funds"
        );

        let mut actual_amount = amount;
        let mut new_staked = ledger
            .staked
            .checked_sub(&amount)
            .ok_or("Error calculating new staking")?;
        // check if the value is too small to be an oracle or candidate
        if new_staked < T::MinStaking::get() {
            // Unbond remaining funds as a whole
            new_staked = Zero::zero();
            actual_amount = ledger.staked;

            if Self::remove_oracle(who) {
                Self::add_unqualified_member(who);
            } else if Self::remove_candidate(who) {
                Self::add_unqualified_member(who);
            } else {
                return Err("Unexpected error occured");
            }
        }

        ledger.staked = new_staked;
        let new_unbond = Unbond {
            amount: actual_amount,
            until: current_height + T::LockedDuration::get(),
        };

        ledger.unbonds.push(new_unbond);

        <Ledgers<T>>::insert(who, ledger);
        Self::deposit_event(RawEvent::OracleUnbonded(who.clone(), actual_amount));
        Ok(())
    }
}

/// Business module should use this trait to
/// communicate with oracle module in order to decouple them.
pub trait OracleMixedIn<T: system::Trait> {
    /// Create request
    fn create_request(
        from: &T::AccountId,
        meta: &Vec<u8>,
        timeout: T::BlockNumber,
        oracle: &T::AccountId,
    ) -> result::Result<T::Hash, &'static str>;
    /// Cancel request
    fn cancel_request(from: &T::AccountId, id: T::Hash) -> Result;
    /// Called after request is fulfilled
    fn on_request_fulfilled(oracle: &T::AccountId, id: T::Hash) -> Result;
    /// Predicate if one oracle is valid.
    fn is_valid(who: &T::AccountId) -> bool;
}

/// External interface
impl<T: Trait> OracleMixedIn<T> for Module<T> {
    /// Create request
    ///
    /// @from   the initiator
    /// @meta   the meta data for request, will support json serialization
    /// @timeout    the timeout value
    /// @oracle the specified oracle
    fn create_request(
        from: &T::AccountId,
        meta: &Vec<u8>,
        timeout: T::BlockNumber,
        oracle: &T::AccountId,
    ) -> result::Result<T::Hash, &'static str> {
        // Check length of meta
        ensure!(
            meta.len() <= 1024,
            "The length of meta should be equal or less than 1024"
        );
        // Check timeout value TODO: use config value?
        ensure!(
            timeout > T::BlockNumber::min_value() && timeout <= T::MaxTimeout::get(),
            "Invalid timeout range, should be (0, MaxTimeout]"
        );
        // Check if oracle exists or not
        ensure!(Self::oracles().contains(oracle), "Should be a valid oracle");
        // Calculate hash of the request parameters
        let created_at = Self::block_number();
        let expired_at = created_at + timeout;
        let reward = T::OracleFee::get();
        let nonce = Nonce::get();
        // Get hash value of all parameters
        let hash = (from, meta, created_at, expired_at, oracle, reward, nonce)
            .using_encoded(<T as system::Trait>::Hashing::hash);
        // Check if hash value conflicts with previous jobs
        ensure!(!<Jobs<T>>::exists(hash), "Hash value already exists");
        // Transfer funds for fee
        T::Currency::transfer(&from, &Self::cashier_account(), T::OracleFee::get())?;

        let job = JobOf::<T> {
            from: from.clone(),
            meta: meta.clone(),
            created_at: created_at,
            expired_at: expired_at,
            oracle: oracle.clone(),
            reward: T::OracleFee::get(),
            nonce: nonce,
        };
        // Everything is ok, do the actual insertion now
        <Jobs<T>>::insert(hash, job);
        let mut info = Self::oracle_info(oracle.clone());
        info.total_jobs += 1;
        <OracleInfos<T>>::insert(oracle.clone(), info);

        Nonce::mutate(|n| *n += 1);

        Self::deposit_event(RawEvent::JobCreated(
            from.clone(),
            oracle.clone(),
            created_at,
            hash,
        ));

        Ok(hash)
    }

    /// Cancel pending request
    ///
    /// @from   the original initiator
    /// @id     the hash of request
    fn cancel_request(from: &T::AccountId, id: T::Hash) -> Result {
        ensure!(<Jobs<T>>::exists(id), "Job does not exist");
        let job = Self::job(id);
        ensure!(job.from == from.clone(), "Not authorized");
        let block_number = Self::block_number();
        ensure!(job.expired_at <= block_number, "Job is not expired");
        // Take back oracle fee 
        T::Currency::transfer(&Self::cashier_account(), &from, T::OracleFee::get())?;
        // Update and send event notification
        let mut info = Self::oracle_info(job.oracle.clone());
        info.total_missed_jobs += 1;
        <OracleInfos<T>>::insert(job.oracle.clone(), info);

        <Jobs<T>>::remove(id);
        Self::deposit_event(RawEvent::JobCancelled(from.clone(), block_number, id));

        Ok(())
    }

    /// Called after request is fulfilled
    ///
    /// @oralce: the account of oracle
    /// @id: id of the requested job
    fn on_request_fulfilled(oracle: &T::AccountId, id: T::Hash) -> Result {
        ensure!(<Jobs<T>>::exists(id), "Job does not exist");
        let job = Self::job(id);
        ensure!(job.oracle == oracle.clone(), "Not authorized");

        let block_number = Self::block_number();
        let mut info = Self::oracle_info(oracle.clone());

        ensure!(block_number < job.expired_at, "Job already expired");

        info.total_witnessed_jobs += 1;
        // TODO: delay transfer?
        info.total_reward = info.total_reward.saturating_add(job.reward);
        info.withdrawable_reward = info.withdrawable_reward.saturating_add(job.reward);
       
        // Update send event notification
        <OracleInfos<T>>::insert(oracle.clone(), info);
        <Jobs<T>>::remove(id);

        Self::deposit_event(RawEvent::JobFulfilled(oracle.clone(), block_number, id));
        <WitnessReport<T>>::insert(oracle.clone(), block_number);
        Ok(())
    }

    /// Check if the account report is valid or not
    ///
    /// @who the account
    fn is_valid(who: &T::AccountId) -> bool {
        // let report_height = Self::witness_report(who);
        // report_height + T::ReportInteval::get() >= Self::block_number()
        true
    }
}

/// Help functions go here
impl<T: Trait> Module<T> {
    /// Get current block number
    fn block_number() -> T::BlockNumber {
        <system::Module<T>>::block_number()
    }
}
