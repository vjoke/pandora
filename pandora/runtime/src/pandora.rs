//! # Pandora
//!
//! `pandora` is a module for gaming, we use this module to IGO(Initial Gaming Offering) our tokens

use support::{
    decl_event, decl_module, decl_storage,
    dispatch::Result,
    ensure,
    traits::{Currency, Get, Imbalance, ReservableCurrency},
    StorageMap, StorageValue,
};

use codec::{Decode, Encode};
use rstd::prelude::*;
use sr_primitives::traits::{Hash, Saturating, Zero};
use system::ensure_signed;

/// Status defines the game status
/// # Status
/// - Inited    The game is inited
/// - Running   The game is running, players can create box, upgrade it or open it
/// - Settling  The game has expired, and system is settling the pending bonus and prize
/// - Stopped   The game is stopped
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq)]
pub enum Status {
    None,
    Inited,
    Running,
    Settling,
    Paused,
    Stopped,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}

/// The status definition for dbox
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq)]
pub enum DboxStatus {
    None,
    /// The dbox is locked
    Locked(u64),
    /// The dbox is created
    Active,
    /// The dbox is opening
    Opening,
    /// The dbox is opened
    Opened,
}

impl Default for DboxStatus {
    fn default() -> Self {
        DboxStatus::None
    }
}

/// The dbox struct
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Dbox<Hash, Balance, AccountId> {
    /// The hash of the dbox
    pub id: Hash,
    /// The position at which the dbox is created
    pub create_position: u64,
    /// The status of dbox
    pub status: DboxStatus,
    /// The accumulated bonus
    pub value: Balance,
    /// The version of dbox
    pub version: u64,
    /// The invitor of the dbox
    pub invitor: Option<AccountId>,
    /// The position when dbox is requested to open
    pub open_position: u64,
    /// The per-dbox bonus
    pub bonus_per_dbox: Balance,
    /// The bonus position of this dbox, we use it to keep the pending bonus dbox position
    pub bonus_position: u64,
}

/// The status of player
#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq)]
pub enum PlayerStatus {
    None,
    /// The player is active
    Active,
    /// The player is forbidden
    Forbidden,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        PlayerStatus::None
    }
}

/// The player struct
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Player<Balance> {
    /// Total bonus received
    pub total_bonus: Balance,
    /// Total prize received
    pub total_prize: Balance,
    /// Total commission received
    pub total_commission: Balance,
    /// Player status
    pub status: PlayerStatus,
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
    /// Define the expiration in seconds for one round of game
    type Expiration: Get<u32>;
    /// Max latest dboxes to share the money of prize pool
    type MaxLatest: Get<u64>;
    /// Define min unit price for dbox
    type MinUnitPrice: Get<BalanceOf<Self>>;
    /// Define max unit price of dbox
    type MaxUnitPrice: Get<BalanceOf<Self>>;
    /// The bonus ratio for previous active dbox
    type DboxRatio: Get<u32>;
    /// The reserve ratio for the dbox
    type ReserveRatio: Get<u32>;
    /// The prize pool ratio for the dbox
    type PoolRatio: Get<u32>;
    /// The ratio for last player
    type LastPlayerRatio: Get<u32>;
    /// The ratio for the team
    type TeamRatio: Get<u32>;
    /// The operator ratio
    type OperatorRatio: Get<u32>;
    /// The invitor ratio
    type InvitorRatio: Get<u32>;
    /// The currency type
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type DboxOf<T> = Dbox<<T as system::Trait>::Hash, BalanceOf<T>, <T as system::Trait>::AccountId>;
type PlayerOf<T> = Player<BalanceOf<T>>;

decl_event!(
    pub enum Event<T>
    where
        Hash = <T as system::Trait>::Hash,
        BlockNumber = <T as system::Trait>::BlockNumber,
        AccountId = <T as system::Trait>::AccountId,
    {
        /// New dbox is created
        DboxCreated(Hash, AccountId),
        /// Dobx is opening
        DboxOpening(Hash, AccountId),
        /// Dobx is opened
        DboxOpened(Hash),
        /// Dbox is upgraded
        DboxUpgraded(Hash, AccountId),
        /// Game is inited
        GameInited(BlockNumber, AccountId),
        /// Game is inited
        GameRunning(BlockNumber, Option<AccountId>),
        /// Game is settling
        GameSettling(BlockNumber),
        /// Game is stopped
        GameStopped(BlockNumber, AccountId),
    }
);

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as PandoraModule {
        /// The admin account
        AdminAccount get(admin_account) config(): T::AccountId;
        /// The cashier account
        CashierAccount get(cashier_account) config(): T::AccountId;
        /// The reserve account
        ReserveAccount get(reserve_account) config(): T::AccountId;
        /// The pool account
        PoolAccount get(pool_account) config(): T::AccountId;
        /// The last player account
        LastPlayerAccount get(last_player_account) config(): T::AccountId;
        /// The team account
        TeamAccount get(team_account) config(): T::AccountId;
        /// The operator account
        OperatorAccount get(operator_account) config(): T::AccountId;
        /// Ledger is used to keep balance of each account
        Ledger get(balance): map T::AccountId => BalanceOf<T>;
        /// The game status
        GameStatus get(game_status): Status;
        /// The timeout remained
        Timeout get(timeout): u32;
        /// The unit price of dbox
        DboxUnitPrice get(dbox_unit_price): BalanceOf<T>;
        /// The round number
        RoundCount get(round_count): u64;
        // The start position of current round
        RoundStartDbox get(round_start_dbox): u64;
        // The bonus dbox position of current round
        BonusDbox get(bonus_dbox): u64;
        // All the dboxes
        DboxOwner get(owner_of): map T::Hash => Option<T::AccountId>;
        /// The active dbox count
        AllActiveDboxesCount get(all_active_dboxes_count): u64;
        /// The maximum active dboxes
        MaxActiveDboxesCount get(max_active_dboxes_count): u64;
        /// The preset maximum active dboxes
        MaxPresetActiveDboxesCount get(max_preset_active_dboxes_count): u64;
        /// All dboxes array
        AllDboxesArray get(dbox_by_index): map u64 => DboxOf<T>;
        /// All dboxes count
        AllDboxesCount get(all_dboxes_count): u64;
        /// The map for hash to position in array of dboxes
        AllDboxesIndex: map T::Hash => u64;
        /// All opening dboxes
        /// Get index of array by create positon
        AllOpeningDboxesMap get(opening_dbox_by_position): map u64 => u64;
        /// Get create position by index of array
        AllOpeningDboxesArray get(opening_dbox_by_index): map u64 => u64;
        /// All opening dboxes count
        AllOpeningDboxesCount get(all_opening_doxes_count): u64;
        /// The owned dboxes array
        OwnedDboxesArray get(dbox_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        /// The count of dbox owned by some a account
        OwnedDboxesCount get(owned_dbox_count): map T::AccountId => u64;
        /// The map between hash to position of dboxes for some a account
        OwnedDboxesIndex: map T::Hash => u64;
        /// Latest dboxes
        LatestDboxes get(latest_dbox_by_index): map u64 => (T::AccountId, u64);
        /// The last dboxes index
        LastDboxIndex get(last_dbox_index): u64;
        /// The count of latest dboxes
        LatestDboxesCount get(latest_dboxes_count): u64;
        /// The count of latest dboxes which have received prize
        ReleasedDboxesCount get(released_dboxes_count): u64;
        /// The average prize for latest dboxes
        AveragePrize get(average_prize): BalanceOf<T>;
        // All Players
        AllPlayers get(player): map T::AccountId => PlayerOf<T>;
        /// The count of all players
        AllPlayersCount get(player_count): u64;
        /// The maximus ops for each block
        MaxOps get(max_ops): u32;
        /// The nonce value for hash of dbox
        Nonce: u64;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        // Expiration for a round
        const Expiration: u32 = T::Expiration::get();
        const MaxLatest: u64 = T::MaxLatest::get();
        // Price limitation
        const MinUnitPrice: BalanceOf<T> = T::MinUnitPrice::get();
        const MaxUnitPrice: BalanceOf<T> = T::MaxUnitPrice::get();
        // Ratios for bonus
        const DboxRatio: u32 = T::DboxRatio::get();
        const ReserveRatio: u32 = T::ReserveRatio::get();
        const PoolRatio: u32 = T::PoolRatio::get();
        const LastPlayerRatio: u32 = T::LastPlayerRatio::get();
        const TeamRatio: u32 = T::TeamRatio::get();
        const OperatorRatio: u32 = T::OperatorRatio::get();
        const InvitorRatio: u32 = T::InvitorRatio::get();

        /// Initialize the game with price
        ///
        /// @origin
        /// @dbox_unit_price    the price of dbox
        pub fn init(origin, dbox_unit_price: BalanceOf<T>) -> Result {
            // Check signature
            let sender = ensure_signed(origin)?;
            // Check priviledge
            ensure!(sender == Self::admin_account(), "Not authorized");
            ensure!(!GameStatus::exists(), "Already inited");
            // Check unit price
            ensure!(dbox_unit_price > T::MinUnitPrice::get(), "Unit price is too low");
            ensure!(dbox_unit_price <= T::MaxUnitPrice::get(), "Unit price is too high");
            // Init each account of ledger
            let accounts: Vec<T::AccountId> = vec![Self::admin_account(), Self::cashier_account(), Self::reserve_account(),
                Self::pool_account(), Self::last_player_account(), Self::team_account(), Self::operator_account()];

            for account in accounts.iter() {
                let balance = <BalanceOf<T>>::zero();
                <Ledger<T>>::insert(account, balance);
            }
            // TODO: config?
            let default_max_value = 1000;
            MaxActiveDboxesCount::put(&default_max_value);
            MaxPresetActiveDboxesCount::put(&default_max_value);

            GameStatus::put(Status::Inited);
            Timeout::put(T::Expiration::get());
            <DboxUnitPrice<T>>::put(dbox_unit_price);
            RoundCount::put(1);
            RoundStartDbox::put(0);
            BonusDbox::put(0);
            <AveragePrize<T>>::put(<BalanceOf<T>>::zero());
            MaxOps::put(100);
            // Trigger event
            Self::deposit_event(RawEvent::GameInited(Self::block_number(), sender.clone()));
            Ok(())
        }

        /// Set the new status for the game
        ///
        /// @origin
        /// @new_status new status of the system
        // pub fn set_status(origin, new_status: Status) -> Result {
        //     let sender = ensure_signed(origin)?;
        //     ensure!(sender == Self::admin_account(), "Not authorized");
        //     ensure!(GameStatus::exists(), "Not inited");
        //     ensure!(new_status != Status::Inited, "Invalid new status");
        //     ensure!(new_status != GameStatus::get(), "New status should be different from current status");

        //     GameStatus::put(new_status);
        //     let block_number = Self::block_number();
        //     // Trigger event
        //     match new_status {
        //         Status::Running => Self::deposit_event(RawEvent::GameRunning(block_number, Some(sender.clone()))),
        //         Status::Stopped => Self::deposit_event(RawEvent::GameStopped(block_number, sender.clone())),
        //         _ => (),
        //     }

        //     Ok(())
        // }

        /// Set the new status for the game
        ///
        /// @origin
        /// @value new status of the system
        pub fn set_status(origin, value: u8) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::admin_account(), "Not authorized");
            ensure!(GameStatus::exists(), "Not inited");

            let new_status: Status = match value {
                0 => Status::None,
                1 => Status::Inited,
                2 => Status::Running,
                3 => Status::Settling,
                4 => Status::Paused,
                5 => Status::Stopped,
                _ => return Err("Invalid status value"),
            };

            ensure!(new_status != Status::Inited, "Invalid new status");
            ensure!(new_status != GameStatus::get(), "New status should be different from current status");

            GameStatus::put(new_status);
            let block_number = Self::block_number();
            // Trigger event
            match new_status {
                Status::Running => Self::deposit_event(RawEvent::GameRunning(block_number, Some(sender.clone()))),
                Status::Stopped => Self::deposit_event(RawEvent::GameStopped(block_number, sender.clone())),
                _ => (),
            }

            Ok(())
        }

        /// Set the maximus ops for each block
        ///
        /// @origin
        /// @new_max_ops new max ops
        pub fn set_max_ops(origin, new_max_ops: u32) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::admin_account(), "Not authorized");
            ensure!(GameStatus::exists(), "Not inited");
            ensure!(new_max_ops != Self::max_ops(), "New value should be different from current value");
            ensure!(new_max_ops > 0 && new_max_ops <= 10_000, "Invalid range"); // FIXME: (0, 10_000]

            MaxOps::put(new_max_ops);
            Ok(())
        }

        /// Preset max active dboxes for the game
        ///
        /// @origin
        /// @max_active_dboxes_count    maximum active dboxes permitted
        pub fn preset_max_active_dboxes_count(origin, max_active_dboxes_count: u64) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::admin_account(), "Not authorized");
            ensure!(max_active_dboxes_count > 0 && max_active_dboxes_count < 1_000_000, "Invalid preset max active dboxes count");
            ensure!(max_active_dboxes_count != Self::max_preset_active_dboxes_count(), "New value should be different from current value");

            MaxPresetActiveDboxesCount::put(max_active_dboxes_count);
            Ok(())
        }

        /// Create a dbox
        ///
        /// @origin the creator
        /// @invitor the invitor of the new dbox
        // pub fn create_dbox(origin, invitor: Option<T::AccountId>) -> Result {
        //     let sender = ensure_signed(origin)?;
        //     let _ = Self::check_inviting(&invitor, &sender)?;
        //     let _ = Self::ensure_status(vec![Status::Running])?;
        //     // Check if the account is system account
        //     ensure!(!<Ledger<T>>::exists(&sender), "System account is not allowed");

        //     let _ = Self::do_create_dbox(&sender, invitor, true)?;
        //     // TODO: cashier_account?
        //     Ok(())
        // }

        /// Create a dbox
        ///
        /// @origin the creator
        pub fn create_dbox(origin) -> Result {
            let invitor = None;
            let sender = ensure_signed(origin)?;
            let _ = Self::check_inviting(&invitor, &sender)?;
            let _ = Self::ensure_status(vec![Status::Running])?;
            // Check if the account is system account
            ensure!(!<Ledger<T>>::exists(&sender), "System account is not allowed");

            let _ = Self::do_create_dbox(&sender, invitor, true)?;
            // TODO: cashier_account?
            Ok(())
        }

        /// Create a dbox with invitor
        ///
        /// @origin the creator
        /// @invitor the invitor of the new dbox
        pub fn create_dbox_with_invitor(origin, invitor: Option<T::AccountId>) -> Result {
            let sender = ensure_signed(origin)?;
            let _ = Self::check_inviting(&invitor, &sender)?;
            let _ = Self::ensure_status(vec![Status::Running])?;
            // Check if the account is system account
            ensure!(!<Ledger<T>>::exists(&sender), "System account is not allowed");

            let _ = Self::do_create_dbox(&sender, invitor, true)?;
            // TODO: cashier_account?
            Ok(())
        }

        /// Open the dbox, the following requirements should be met
        ///
        /// 1. has right signature
        /// 2. box does exist
        /// 3. the owner of the dbox is the sender
        /// 4. the status is active
        /// 5. game status is running, settling or paused
        ///
        /// @origin
        /// @dbox_id    the dbox id
        pub fn open_dbox(origin, dbox_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            Self::open_dbox_by_id(&sender, dbox_id)
        }

        /// Open dbox by index
        ///
        /// @origin
        /// @index  the index of dbox owned by origin
        pub fn open_dbox_by_index(origin, index: u64) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<OwnedDboxesArray<T>>::exists((sender.clone(), index)), "Dbox does not exist");
            let dbox_id = Self::dbox_of_owner_by_index((sender.clone(), index));
            Self::open_dbox_by_id(&sender, dbox_id)
        }

        /// Upgrade dbox, the following requirements should be met
        ///
        /// @origin
        /// @dbox_id    the dbox id
        pub fn upgrade_dbox(origin, dbox_id: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            Self::upgrade_dbox_by_id(&sender, dbox_id)
        }

        /// Upgrade dbox by index
        ///
        /// @origin
        /// @dbox_id    the dbox id
        pub fn upgrade_dbox_by_index(origin, index: u64) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(<OwnedDboxesArray<T>>::exists((sender.clone(), index)), "Dbox does not exist");
            let dbox_id = Self::dbox_of_owner_by_index((sender.clone(), index));

            Self::upgrade_dbox_by_id(&sender, dbox_id)
        }

        /// Callback when a block is finalized
        ///
        /// @n  the block number
        fn on_finalize(_n: T::BlockNumber) {
            let game_status = GameStatus::get();
            // Check status
            if game_status != Status::Running && game_status != Status::Settling {
                return;
            }

            let mut ops:i32 = Self::max_ops() as i32;
            // Update game status
            if GameStatus::get() == Status::Running {
                let mut timeout = Self::timeout();
                timeout = timeout.saturating_sub(10); // TODO: use config value
                if timeout == 0 {
                    let _ = Self::begin_settling();
                }
                Timeout::mutate(|n| *n = timeout);
            }
            // Loop to drain pending bonus
            loop {
                if ops <= 0 {
                    break;
                }

                if !Self::drain_bonus(&mut ops) {
                    break;
                }
            }

            // Loop to send money to latest boxes
            if GameStatus::get() == Status::Settling {
                loop {
                    if ops <= 0 {
                        break;
                    }

                    if !Self::release_prize(&mut ops) {
                        let _ = Self::end_settling();
                        break;
                    }
                }
            }
        }
    }
}

impl<T: Trait> Module<T> {
    /// Check if inviting is ok or not
    ///
    /// @invitor_account    the invitor account
    /// @invitee    the invitee
    fn check_inviting(invitor_account: &Option<T::AccountId>, invitee: &T::AccountId) -> Result {
        if let Some(invitor) = invitor_account {
            // Make sure invitor is from the owner of previous creatd dbox
            ensure!(
                <AllPlayers<T>>::exists(invitor),
                "Invitor should be the player"
            );
            let invitor_player = Self::player(invitor);
            ensure!(
                invitor_player.status == PlayerStatus::Active,
                "Invitor should be active player"
            );

            // Make sure invitee does not exists
            ensure!(
                !<AllPlayers<T>>::exists(invitee),
                "Invitee should be a new player"
            );
            // TODO: add blacklist
        }

        Ok(())
    }

    /// ensure status is ok or not
    ///
    /// @status_vec the vector of statuses expected
    fn ensure_status(status_vec: Vec<Status>) -> Result {
        let current_status = GameStatus::get();
        ensure!(
            status_vec.iter().any(|status| current_status == *status),
            "Status is not ready"
        );
        Ok(())
    }

    /// Check if the dbox is ok for inserting
    ///
    /// @from   the creator of the dbox
    /// @dbox_id    the id of dbox
    fn check_insert(from: &T::AccountId, dbox_id: &T::Hash) -> Result {
        ensure!(!<DboxOwner<T>>::exists(dbox_id), "Dbox already exists");

        let owned_dbox_count = Self::owned_dbox_count(from);
        let _new_owned_dbox_count = owned_dbox_count
            .checked_add(1)
            .ok_or("Overflow adding a new dbox to account balance")?;

        let all_dboxes_count = Self::all_dboxes_count();
        let _new_all_dboxes_count = all_dboxes_count
            .checked_add(1)
            .ok_or("Overflow adding a new dbox to total supply")?;

        // limit checking
        let all_active_dboxes_count = Self::all_active_dboxes_count();
        let new_all_active_dboxes_count = all_active_dboxes_count
            .checked_add(1)
            .ok_or("Overflow adding a new dbox to total active dboxes")?;
        ensure!(
            new_all_active_dboxes_count <= Self::max_active_dboxes_count(),
            "Exceed max active dboxes limitation"
        );

        Ok(())
    }

    /// Get dbox by hash id
    ///
    /// @dbox_id    the id of dbox
    fn get_dbox_by_id(dbox_id: T::Hash) -> Option<DboxOf<T>> {
        // FIXME: what if the dbox does not exists?
        let index = <AllDboxesIndex<T>>::get(dbox_id);
        Some(<AllDboxesArray<T>>::get(index))
    }

    /// Insert the new player if does not exist
    ///
    /// @player_account the player account
    fn may_insert_new_player(player_account: &T::AccountId) -> Result {
        if !<AllPlayers<T>>::exists(player_account) {
            let player = PlayerOf::<T> {
                total_bonus: Zero::zero(),
                total_prize: Zero::zero(),
                total_commission: Zero::zero(),
                status: PlayerStatus::Active,
            };

            <AllPlayers<T>>::insert(player_account, player);
            AllPlayersCount::mutate(|n| *n += 1);
        }

        Ok(())
    }

    /// Split fund of the dbox
    ///
    /// @new_dbox   the newly dbox created
    fn split_money(new_dbox: &mut DboxOf<T>) -> Result {
        let money = Self::dbox_unit_price() / 100.into();
        // Fill bonus info for all active dboxes if any
        let all_active_dboxes_count = Self::all_active_dboxes_count();
        if all_active_dboxes_count > 0 {
            let bonus_amount = money.saturating_mul(T::DboxRatio::get().into());
            // FIXME: TODO: support u64?
            new_dbox.bonus_per_dbox = bonus_amount / (all_active_dboxes_count as u32).into();
        }
        // Give to other game acounts
        let targets: Vec<(T::AccountId, u32)> = vec![
            (Self::reserve_account(), T::ReserveRatio::get()),
            (Self::pool_account(), T::PoolRatio::get()),
            (Self::last_player_account(), T::LastPlayerRatio::get()),
            (Self::team_account(), T::TeamRatio::get()),
            (Self::operator_account(), T::OperatorRatio::get()),
        ];

        for t in targets.iter() {
            let account = &t.0;
            let balance = <Ledger<T>>::get(account);
            let amount = money.saturating_mul(t.1.into());
            let new_balance = balance.saturating_add(amount);
            <Ledger<T>>::insert(account, new_balance);
        }
        // Send commission to invitor directly
        if let Some(invitor_account) = &new_dbox.invitor {
            let commission_amount = money.saturating_mul(T::InvitorRatio::get().into());
            T::Currency::transfer(
                &Self::cashier_account(),
                &invitor_account,
                commission_amount,
            )?;
            // Update invitor's commission balance
            Self::add_commission(&invitor_account, commission_amount)?;
        }

        Ok(())
    }

    /// Substract balance of system account
    ///
    /// @account    the accout whose balance is going to be substracted
    /// @amount the value to be substracted
    fn substract_balance(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
        let balance = <Ledger<T>>::get(account);
        let new_balance = balance.saturating_sub(amount); // FIXME: check saturating_sub
        <Ledger<T>>::insert(account, new_balance);
        Ok(())
    }

    /// Add the bonus
    ///
    /// @account
    /// @amount
    fn add_bonus(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
        let mut player = Self::player(account);
        player.total_bonus = player.total_bonus.saturating_add(amount);
        <AllPlayers<T>>::insert(account, player);

        Ok(())
    }

    /// Add the prize
    ///
    /// @account
    /// @amount
    fn add_prize(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
        let mut player = Self::player(account);
        player.total_prize = player.total_prize.saturating_add(amount);
        <AllPlayers<T>>::insert(account, player);

        Ok(())
    }

    /// Add the commission
    ///
    /// @account
    /// @amount
    fn add_commission(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
        let mut player = Self::player(account);
        player.total_commission = player.total_commission.saturating_add(amount);
        <AllPlayers<T>>::insert(account, player);

        Ok(())
    }

    /// Insert the new dbox
    ///
    /// @from the dbox creator
    /// @dbox_id the dbox id
    /// @new_dbox the dbox struct
    fn insert_dbox(from: &T::AccountId, dbox_id: T::Hash, new_dbox: &DboxOf<T>) -> Result {
        let _ = Self::check_insert(from, &dbox_id)?;

        let owned_dbox_count = Self::owned_dbox_count(from);
        let new_owned_dbox_count = owned_dbox_count
            .checked_add(1)
            .ok_or("Overflow adding a new dbox to account balance")?;

        let all_dboxes_count = Self::all_dboxes_count();
        let new_all_dboxes_count = all_dboxes_count
            .checked_add(1)
            .ok_or("Overflow adding a new dbox to total supply")?;

        // Sanity checking
        let all_active_dboxes_count = Self::all_active_dboxes_count();
        let new_all_active_dboxes_count = all_active_dboxes_count
            .checked_add(1)
            .ok_or("Overflow adding a new dbox to total active dboxes")?;

        <DboxOwner<T>>::insert(dbox_id, from);
        AllActiveDboxesCount::put(new_all_active_dboxes_count);

        <AllDboxesArray<T>>::insert(all_dboxes_count, new_dbox);
        AllDboxesCount::put(new_all_dboxes_count);
        <AllDboxesIndex<T>>::insert(dbox_id, all_dboxes_count);

        <OwnedDboxesArray<T>>::insert((from.clone(), owned_dbox_count), dbox_id);
        <OwnedDboxesCount<T>>::insert(from, new_owned_dbox_count);
        <OwnedDboxesIndex<T>>::insert(dbox_id, owned_dbox_count);

        Self::deposit_event(RawEvent::DboxCreated(dbox_id, from.clone()));

        Ok(())
    }

    /// Send pending bonus
    ///
    /// @dbox
    fn send_pending_bonus(dbox: &DboxOf<T>) -> Result {
        let mut prev_dbox = Self::dbox_by_index(dbox.bonus_position);
        if prev_dbox.status == DboxStatus::Active || prev_dbox.status == DboxStatus::Opening {
            prev_dbox.value += dbox.bonus_per_dbox;
            // FIXME:
            if prev_dbox.status == DboxStatus::Opening
                && prev_dbox.open_position == dbox.create_position
            {
                Self::do_open_dbox(&mut prev_dbox, true, true)?;
            }
            // Update dbox
            <AllDboxesArray<T>>::insert(prev_dbox.create_position, prev_dbox);
        }

        Ok(())
    }

    /// Check if the dbox has pending bonus or not
    ///
    /// @dbox
    fn has_pending_bonus(dbox: &DboxOf<T>) -> bool {
        if Self::is_staled_dbox(dbox) {
            return false;
        }
        if Self::bonus_dbox() == Self::all_dboxes_count() {
            return false;
        }
        true
    }

    /// Get pending bonus and flag to indicate if player will get double prize
    ///
    /// @dbox
    /// @return (has_pending, double)
    pub fn get_pending_bonus(dbox: &DboxOf<T>) -> (bool, bool) {
        let status = GameStatus::get();

        if Self::is_staled_dbox(dbox) {
            return (false, false);
        }

        let mut double = false;
        if status == Status::Running {
            double = true;
        }

        if Self::bonus_dbox() == Self::all_dboxes_count() {
            return (false, double);
        }

        (true, double)
    }

    /// Check if the dbox is staled or not
    ///
    /// @dbox   the dobx to be checked
    fn is_staled_dbox(dbox: &DboxOf<T>) -> bool {
        let round_start_dbox = Self::round_start_dbox();
        dbox.create_position < round_start_dbox
    }

    /// Create a new dbox with tokens or bonus of old dboxes
    ///
    /// @sender the creator the dbox
    /// @invitor the invitor
    /// @transfer true if transfer token, otherwise false
    fn do_create_dbox(
        sender: &T::AccountId,
        invitor: Option<T::AccountId>,
        transfer: bool,
    ) -> Result {
        // Generate hash to assign id of dbox
        let nonce = Nonce::get();
        let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
            .using_encoded(<T as system::Trait>::Hashing::hash);

        let mut new_dbox = DboxOf::<T> {
            id: random_hash,
            create_position: Self::all_dboxes_count(),
            status: DboxStatus::Active,
            value: Zero::zero(),
            version: 0,
            invitor: invitor,
            open_position: 0, // FIXME:
            bonus_per_dbox: Zero::zero(),
            bonus_position: Self::round_start_dbox(),
        };
        // Check if we can insert dbox without error
        let _ = Self::check_insert(&sender, &random_hash)?;
        if transfer {
            // Transfer fund of buying dbox to our cashier account
            let _ =
                T::Currency::transfer(&sender, &Self::cashier_account(), Self::dbox_unit_price())?;
        }
        // From now on, all state transition operations should be infailable
        Self::split_money(&mut new_dbox)?;
        Self::insert_dbox(&sender, random_hash, &new_dbox)?;
        Self::on_dbox_operation(&sender, &new_dbox)?;
        Self::may_insert_new_player(&sender)?;
        // Change nonce value to introduce random value
        Nonce::mutate(|n| *n += 1);
        Ok(())
    }

    /// Add opening dbox to array
    ///
    /// @dbox the dbox to be opened later
    fn add_opening_dbox(dbox: &DboxOf<T>) -> Result {
        let all_opening_dboxes_count = Self::all_opening_doxes_count();
        let new_all_opening_dboxes_count = all_opening_dboxes_count
            .checked_add(1)
            .ok_or("Overflow adding a new opening dbox")?;

        AllOpeningDboxesMap::insert(dbox.create_position, all_opening_dboxes_count);
        AllOpeningDboxesArray::insert(all_opening_dboxes_count, dbox.create_position);
        // Update count
        AllOpeningDboxesCount::put(new_all_opening_dboxes_count);

        Ok(())
    }

    /// Open dbox by id
    ///
    /// @sender the player
    /// @dbox_id    id of the dbox
    fn open_dbox_by_id(sender: &T::AccountId, dbox_id: T::Hash) -> Result {
        let _ = Self::ensure_status(vec![Status::Running, Status::Settling, Status::Paused])?;
        ensure!(<DboxOwner<T>>::exists(dbox_id), "Dbox does not exist");
        ensure!(
            Some(sender.clone()) == <DboxOwner<T>>::get(dbox_id),
            "The owner of the dbox is not the sender"
        );

        let mut dbox = Self::get_dbox_by_id(dbox_id).unwrap();
        ensure!(
            dbox.status == DboxStatus::Active,
            "The status of dbox should be active"
        );
        // Mark open position
        dbox.open_position = Self::all_dboxes_count();
        dbox.status = DboxStatus::Opening;
        // FIXME: Double checking
        let (has_pending, double) = Self::get_pending_bonus(&dbox);
        if has_pending {
            Self::add_opening_dbox(&dbox)?;
        } else {
            Self::do_open_dbox(&mut dbox, double, false)?;
        }
        // Save status
        <AllDboxesArray<T>>::insert(dbox.create_position, &dbox);
        // Update counter for running game
        if double {
            let all_active_dboxes_count = Self::all_active_dboxes_count();
            let new_all_active_dboxes_count = all_active_dboxes_count
                .checked_sub(1)
                .ok_or("Underflow substracting a dbox from total active dboxes")?;

            AllActiveDboxesCount::put(new_all_active_dboxes_count);
            Self::on_dbox_operation(&sender, &dbox)?;
        }
        // Trigger events
        if !has_pending {
            Self::deposit_event(RawEvent::DboxOpening(dbox.id, sender.clone()));
        }

        Ok(())
    }

    /// Remove opening dbox from array
    ///
    /// @dbox_position the create position of dbox
    fn remove_opening_dbox(create_position: u64) -> Result {
        ensure!(
            AllOpeningDboxesMap::exists(create_position),
            "The opening dbox does not existed"
        );
        let index = Self::opening_dbox_by_position(create_position);

        let all_opening_dboxes_count = Self::all_opening_doxes_count();
        let largest_index = all_opening_dboxes_count
            .checked_sub(1)
            .ok_or("Underflow removing an opening dbox")?;
        // Swap
        let member_to_remove = Self::opening_dbox_by_index(index);
        if index != largest_index {
            let temp_position = Self::opening_dbox_by_index(largest_index);
            AllOpeningDboxesArray::insert(index, temp_position);
            AllOpeningDboxesArray::insert(largest_index, member_to_remove);

            AllOpeningDboxesMap::insert(temp_position, index);
            AllOpeningDboxesMap::insert(create_position, largest_index);
        }
        // Pop
        AllOpeningDboxesMap::remove(create_position);
        AllOpeningDboxesArray::remove(largest_index);
        // Update count
        AllOpeningDboxesCount::put(largest_index);

        Ok(())
    }

    /// Settle money for the opened dbox
    ///
    /// @dobx the dbox to be opened
    /// @double true if the value will be doubled, otherwise false
    /// @remove true if the opening box will be removed from array, otherwise false
    fn do_open_dbox(dbox: &mut DboxOf<T>, double: bool, remove: bool) -> Result {
        // FIXME: we do not care if transfer is ok or not
        if let Some(player) = Self::owner_of(dbox.id) {
            if !dbox.value.is_zero() {
                let amount = if double {
                    dbox.value.saturating_add(dbox.value)
                } else {
                    dbox.value
                };
                // FIXME: we do not care if the transfer is ok or not
                let _ = T::Currency::transfer(&Self::cashier_account(), &player, amount);
                let _ = Self::add_bonus(&player, amount)?;

                if double {
                    let _ = Self::substract_balance(&Self::reserve_account(), dbox.value)?;
                }
            }
        }

        if remove {
            let _ = Self::remove_opening_dbox(dbox.create_position);
        }

        dbox.value = Zero::zero();
        dbox.status = DboxStatus::Opened;
        Self::deposit_event(RawEvent::DboxOpened(dbox.id));

        Ok(())
    }

    /// Upgrade dbox by id
    /// 
    /// @sender the player
    /// @dbox_id id of the dbox
    fn upgrade_dbox_by_id(sender: &T::AccountId, dbox_id: T::Hash) -> Result {
        let _ = Self::ensure_status(vec![Status::Running])?;

        ensure!(<DboxOwner<T>>::exists(dbox_id), "Dbox does not exist");
        ensure!(Some(sender.clone()) == <DboxOwner<T>>::get(dbox_id), "The owner of the dbox is not the sender");

        let mut dbox = Self::get_dbox_by_id(dbox_id).unwrap();
        ensure!(dbox.status == DboxStatus::Active, "The status of dbox should be active");

        ensure!(dbox.value >= Self::dbox_unit_price(), "Not enough money");

        // Create another new dbox with money in the old dbox
        let _ = Self::do_create_dbox(&sender, None, false)?;
        dbox.value = dbox.value.saturating_sub(Self::dbox_unit_price());
        // Save status
        <AllDboxesArray<T>>::insert(dbox.create_position, &dbox);
        // Trigger event
        Self::deposit_event(RawEvent::DboxUpgraded(dbox.id, sender.clone()));

        Ok(())
    }

    /// Called when box operation occurs
    ///
    /// @player
    /// @dbox
    fn on_dbox_operation(player: &T::AccountId, dbox: &DboxOf<T>) -> Result {
        // Update latest boxes
        let latest_dboxes_count = Self::latest_dboxes_count();
        let last_dbox_index = Self::last_dbox_index();

        if latest_dboxes_count == T::MaxLatest::get() {
            // The queue is full, kick off the first one
            <LatestDboxes<T>>::remove(last_dbox_index - T::MaxLatest::get());
        } else {
            LatestDboxesCount::mutate(|n| *n += 1);
        }

        <LatestDboxes<T>>::insert(last_dbox_index, (player.clone(), dbox.create_position));
        LastDboxIndex::mutate(|n| *n += 1);

        // Increate timeout value
        let mut timeout = Self::timeout();
        timeout += 30;
        timeout = timeout.min(T::Expiration::get());
        Timeout::mutate(|n| *n = timeout);

        Ok(())
    }

    /// Get last dbox
    fn get_last_player() -> Option<T::AccountId> {
        let latest_dboxes_count = Self::latest_dboxes_count();
        let last_dbox_index = Self::last_dbox_index();
        if latest_dboxes_count == 0 {
            return None;
        }
        // Sanity checking
        if !<LatestDboxes<T>>::exists(last_dbox_index - 1) {
            return None;
        }

        Some(<LatestDboxes<T>>::get(last_dbox_index - 1).0)
    }

    /// Reset lastest boxex information
    fn reset_latest_dboxes() -> Result {
        let latest_dboxes_count = Self::latest_dboxes_count();
        let last_dbox_index = Self::last_dbox_index();

        for i in 1..=latest_dboxes_count {
            <LatestDboxes<T>>::remove(last_dbox_index - i);
        }

        LastDboxIndex::mutate(|n| *n = 0);
        LatestDboxesCount::mutate(|n| *n = 0);
        ReleasedDboxesCount::mutate(|n| *n = 0);

        <AveragePrize<T>>::put(<BalanceOf<T>>::zero());

        Ok(())
    }

    /// Begin settling
    fn begin_settling() -> Result {
        // Calculate average prize
        let latest_dboxes_count = Self::latest_dboxes_count();
        if latest_dboxes_count > 0 {
            let money = <Ledger<T>>::get(Self::pool_account());
            // FIXME: less then 100 dboxes
            let prize_amount = money / (latest_dboxes_count as u32).into();
            <AveragePrize<T>>::put(prize_amount);
        }
        // TODO: check if average prize is zero?
        GameStatus::put(Status::Settling);
        // Trigger event
        Self::deposit_event(RawEvent::GameSettling(Self::block_number()));
        Ok(())
    }

    /// Drain bonus
    ///
    /// @ops    operations occured
    fn drain_bonus(ops: &mut i32) -> bool {
        let bonus_dbox = Self::bonus_dbox();
        if bonus_dbox >= Self::all_dboxes_count() {
            return Self::process_opening_dboxes(ops);
        }

        let mut dbox = Self::dbox_by_index(bonus_dbox);
        loop {
            if dbox.bonus_position >= dbox.create_position {
                // Update bonus position
                BonusDbox::put(bonus_dbox + 1);
                *ops -= 1;
                break;
            }
            // Send bonus
            let _ = Self::send_pending_bonus(&dbox);
            dbox.bonus_position += 1; // Move forward

            *ops -= 2;
            if *ops <= 0 {
                break;
            }
        }
        // Update dbox
        <AllDboxesArray<T>>::insert(dbox.create_position, dbox);

        true
    }

    /// Process opening boxes whose open_position is unreachable
    ///
    /// @ops    operations occured
    fn process_opening_dboxes(ops: &mut i32) -> bool {
        loop {
            let all_opening_dboxes_count = Self::all_opening_doxes_count();
            if all_opening_dboxes_count == 0 {
                // We have opened all the pending dboxes
                return false;
            }
            // Get the last opening dbox for performance
            let create_position = AllOpeningDboxesArray::get(all_opening_dboxes_count - 1);
            // Sanity checking?
            let mut dbox = Self::dbox_by_index(create_position);
            assert!(
                dbox.status == DboxStatus::Opening
                    && dbox.open_position == Self::all_dboxes_count(),
                "Status should be opening"
            );
            let _ = Self::do_open_dbox(&mut dbox, true, true);
            // Update dbox
            <AllDboxesArray<T>>::insert(dbox.create_position, dbox);

            *ops -= 2;
            if *ops <= 0 {
                return true;
            }
        }
    }

    /// Release prize for latest boxes
    fn release_prize(ops: &mut i32) -> bool {
        let released_dboxes_count = Self::released_dboxes_count();
        let latest_dboxes_count = Self::latest_dboxes_count();
        let last_dbox_index = Self::last_dbox_index();

        if released_dboxes_count >= latest_dboxes_count {
            if latest_dboxes_count > 0 {
                // Send the last big prize
                if let Some(player) = Self::get_last_player() {
                    // FIXME: We do not care if transfer is ok or not
                    let last_player_account = Self::last_player_account();
                    let last_player_prize = <Ledger<T>>::get(&last_player_account);
                    let _ =
                        T::Currency::transfer(&Self::cashier_account(), &player, last_player_prize);
                    let _ = Self::add_prize(&player, last_player_prize);
                    // Reset last player account
                }
            }
            *ops -= 5;
            return false;
        }

        let i = last_dbox_index - (latest_dboxes_count - released_dboxes_count);
        let (player, _dbox_pos) = <LatestDboxes<T>>::get(i);

        // Share the last prize
        let average_prize = Self::average_prize();
        if !average_prize.is_zero() {
            // FIXME: we don't care if the transfer is ok or not
            let _ = T::Currency::transfer(&Self::cashier_account(), &player, average_prize);
            let _ = Self::add_prize(&player, average_prize);
        }

        ReleasedDboxesCount::mutate(|n| *n += 1);
        *ops -= 3;

        true
    }

    /// End of settling, reset the game and start again, note that all bonus transfer should be finished before reset
    fn end_settling() -> Result {
        // reset round positions
        let all_dboxes_count = Self::all_dboxes_count();

        RoundCount::mutate(|n| *n += 1);
        RoundStartDbox::put(all_dboxes_count);
        BonusDbox::put(all_dboxes_count);
        AllActiveDboxesCount::put(0);

        // FIXME: Reset balances of system accounts
        let accounts: Vec<T::AccountId> = vec![
            Self::cashier_account(),
            Self::reserve_account(),
            Self::pool_account(),
            Self::last_player_account(),
        ];

        for account in accounts.iter() {
            let balance = <BalanceOf<T>>::zero();
            <Ledger<T>>::insert(account, balance);
        }

        // Reset lastest dboxes
        Self::reset_latest_dboxes()?;
        MaxActiveDboxesCount::put(Self::max_preset_active_dboxes_count());
        // Reset status and timeout value
        Timeout::put(T::Expiration::get());
        GameStatus::put(Status::Running);
        // Trigger event
        Self::deposit_event(RawEvent::GameRunning(Self::block_number(), None));

        Ok(())
    }

    /// Get current block number
    fn block_number() -> T::BlockNumber {
        <system::Module<T>>::block_number()
    }
}
