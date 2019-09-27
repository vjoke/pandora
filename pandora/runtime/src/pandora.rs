/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result,
	traits::{Get, Currency, ReservableCurrency, Imbalance}, ensure
};

use system::ensure_signed;
use codec::{Encode, Decode};
use sr_primitives::traits::{Hash, Zero, Saturating, CheckedMul};
use rstd::prelude::*;

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
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

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
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

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Dbox<Hash, Balance, AccountId> {
	/// The hash of the dbox
	id: Hash,
	/// The position at which the dbox is created
	create_position: u64,
	/// The status of dbox 
	status: DboxStatus,
	/// The accumulated value
	value: Balance,
	/// The version of dbox
	version: u64,
	/// The invitor
	invitor: Option<AccountId>,
	/// The position when dbox is opened
	open_position: u64,
	/// Bonus related fields 
	bonus_per_dbox: Balance,
	bonus_position: u64,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq)]
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

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Player<Balance> {
	/// The account statistic information
	total_bonus: Balance,
	total_prize: Balance,
	total_commission: Balance,
	status: PlayerStatus,
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
	// TODO: Add other types and constants required configure this module.
	/// Define the expiration in seconds for one round of game
	type Expiration: Get<u32>; 

	/// Max latest dboxes
	type MaxLatest: Get<u64>;

	/// Define min unit price for dbox
	type MinUnitPrice: Get<BalanceOf<Self>>;
	
	/// Define max unit price of dbox
	type MaxUnitPrice: Get<BalanceOf<Self>>;
	
	/// Define ratios for different parts
	type DboxRatio: Get<u32>;

	type ReserveRatio: Get<u32>;
	type PoolRatio: Get<u32>;
	type LastPlayerRatio: Get<u32>;
	type TeamRatio: Get<u32>;
	type OperatorRatio: Get<u32>;

	type InvitorRatio: Get<u32>;

	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
type DboxOf<T: Trait> = Dbox<T::Hash, BalanceOf<T>, T::AccountId>;
type PlayerOf<T: Trait> = Player<BalanceOf<T>>;

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as PandoraModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(something): Option<u32>;

		AdminAccount get(admin_account) config(): T::AccountId;
		CashierAccount get(cashier_account) config(): T::AccountId;
		ReserveAccount get(reserve_account) config(): T::AccountId;
		PoolAccount get(pool_account) config(): T::AccountId;
		LastPlayerAccount get(last_player_account) config(): T::AccountId;
		TeamAccount get(team_account) config(): T::AccountId;
		OperatorAccount get(operator_account) config(): T::AccountId;
		// Ledger is used to keep balance of each account
		Ledger get(balance): map T::AccountId => BalanceOf<T>;
		
		GameStatus get(game_status): Status;
		Timeout get(timeout): u32;

		DboxUnitPrice get(dbox_unit_price): BalanceOf<T>;
		// The start position of current round
		RoundStartDbox get(round_start_dbox): u64;
		// The bonus dbox position of current round
		BonusDbox get(bonus_dbox): u64;
		// All the dboxes
		DboxOwner get(owner_of): map T::Hash => Option<T::AccountId>;
		/// The active dbox count 
		AllActiveDboxesCount get(all_active_dboxes_count): u64;

		MaxActiveDboxesCount get(max_active_dboxes_count): u64;
		MaxPresetActiveDboxesCount get(max_preset_active_dboxes_count): u64;

		AllDboxesArray get(dbox_by_index): map u64 => DboxOf<T>;
		AllDboxesCount get(all_dboxes_count): u64;
		AllDboxesIndex: map T::Hash => u64;

		OwnedDboxesArray get(dbox_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
		OwnedDboxesCount get(owned_dbox_count): map T::AccountId => u64;
		OwnedDboxesIndex: map T::Hash => u64;

		// Latest box related fields
		LatestDboxes get(latest_dbox_by_index): map u64 => (T::AccountId, u64);
		LastDboxIndex get(last_dbox_index): u64;
		LatestDboxesCount get(latest_dboxes_count): u64;
		ReleasedDboxesCount get(released_dboxes_count): u64;
		AveragePrize get(average_prize): BalanceOf<T>;

		// Players 
		AllPlayers get(player): map T::AccountId => PlayerOf<T>;
		AllPlayersCount get(player_count): u64;

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

		const MinUnitPrice: BalanceOf<T> = T::MinUnitPrice::get();
		const MaxUnitPrice: BalanceOf<T> = T::MaxUnitPrice::get();
		// Ratios
		const DboxRatio: u32 = T::DboxRatio::get();
		const ReserveRatio: u32 = T::ReserveRatio::get();
		const PoolRatio: u32 = T::PoolRatio::get();
		const LastPlayerRatio: u32 = T::LastPlayerRatio::get();
		const TeamRatio: u32 = T::TeamRatio::get();
		const OperatorRatio: u32 = T::OperatorRatio::get();
		const InvitorRatio: u32 = T::InvitorRatio::get();

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Initialize the game with price
		pub fn init(origin, dbox_unit_price: BalanceOf<T>) -> Result {
			// Check signature
			let sender = ensure_signed(origin)?;
			// Check priviledge
			ensure!(sender == Self::admin_account(), "Not authorized");
			ensure!(!GameStatus::exists(), "Already inited");
			// Check unit price
			ensure!(dbox_unit_price > T::MinUnitPrice::get(), "Unit price is too low");
			ensure!(dbox_unit_price < T::MaxUnitPrice::get(), "Unit price is too high");
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
			MaxPreActiveDboxesCount::put(&default_max_value);

			GameStatus::put(Status::Inited);
			Timeout::put(T::Expiration::get());
			<DboxUnitPrice<T>>::put(dbox_unit_price);
			RoundStartDbox::put(0);
			BonusDbox::put(0);
			<AveragePrize<T>>::put(<BalanceOf<T>>::zero());
			// TODO: emit game event
			Ok(())
		}

		/// Set the new status for the game
		pub fn set_status(origin, new_status: Status) -> Result {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::admin_account(), "Not authorized");
			ensure!(new_status != GameStatus::get(), "New status should be different from current status");

			GameStatus::put(new_status);
			// TODO: emit game event
			Ok(())
		}

		/// Preet max active dboxes for the game
		pub fn preset_max_active_dboxes_count(origin, max_active_dboxes_count: u64) -> Result {
			let sender = ensure_signed(origin)?;
			ensure!(sender == Self::admin_account(), "Not authorized");
			ensure!(max_active_dboxes_count > 0 && max_active_dboxes_count < 1_000_000);
			ensure!(max_active_dboxes_count != Self::max_preset_active_dboxes_count(), "New value should be different from current value");

			MaxPresetActiveDboxesCount::put(max_active_dboxes_count);	
			// TODO: emit game event
			Ok(())	
		}

		// Create a dbox
		pub fn create_dbox(origin, invitor: Option<T::AccountId>) -> Result {
			let sender = ensure_signed(origin)?;
			let _ = Self::check_inviting(&invitor, &sender)?;
			let _ = Self::ensure_status(vec![Status::Running])?;
			// Check if the account is priviledged account
			ensure!(!<Ledger<T>>::exists(&sender), "Priviledged account is not allowed");
			
			let _ = Self::do_create_dbox(&sender, &invitor, true)?;
			// TODO: cashier_account?
			Ok(())
		}

		fn do_create_dbox(sender: &T::AccountId, invitor: &Option<T::AccountId>, transfer: bool) -> Result {
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
			let _ = Self::check_insert(&sender, &random_hash, &new_dbox)?;
			if transfer {
				// Transfer fund of buying dbox to our cashier account
				let _ = T::Currency::transfer(&sender, &Self::cashier_account(), Self::dbox_unit_price())?;
			}
			// From now on, all state transition operations should be infailable
			Self::split_money(&mut new_dbox)?;
			Self::insert_dbox(&sender, random_hash, &new_dbox)?;
			Self::on_box_operation(&sender, &new_dbox);
			Self::may_insert_new_player(&sender);
			// Change nonce value to introduce random value
			Nonce::mutate(|n| *n += 1);
			Ok(())
		}

		/// Open the dbox, the following requirements should be met
		/// 1. has right signature
		/// 2. box does exist
		/// 3. the owner of the dbox is the sender
		/// 4. the status is active
		/// 5. game is running
		pub fn open_dbox(origin, dbox_id: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;
			let _ = Self::ensure_status(vec![Status::Running, Status::Settling, Status::Paused])?;
			ensure!(<DboxOwner<T>>::exists(dbox_id), "Dbox does not exist");	
			ensure!(Some(sender.clone()) == <DboxOwner<T>>::get(dbox_id), "The owner of the dbox is not the sender");

			let mut dbox = Self::get_dbox_by_id(dbox_id).unwrap();
			ensure!(dbox.status == DboxStatus::Active, "The status of dbox should be active");
			// Mark open position	
			dbox.open_position = Self::all_dboxes_count();
			dbox.status = DboxStatus::Opening;
			// FIXME: Double checking
			let (has_pending, double) = Self::get_pending_bonus(&dbox);
			if !has_pending {
				Self::do_open_dbox(&mut dbox, double);
			}
			// Save status
			<AllDboxesArray<T>>::insert(dbox.create_position, &dbox);
			
			if Status::Running == GameStatus::get() {
				let all_active_dboxes_count = Self::all_active_dboxes_count();
				let new_all_active_dboxes_count = all_active_dboxes_count.checked_sub(1)
					.ok_or("Overflow substracting a dbox from total active dboxes")?;

				AllActiveDboxesCount::put(new_all_active_dboxes_count);	
				Self::on_box_operation(&sender, &dbox);
			}
			
			Ok(())
		}

		/// Upgrade dbox, the following requirements should be met
		pub fn upgrade_dbox(origin, dbox_id: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;
			let _ = Self::ensure_status(vec![Status::Running])?;

			ensure!(<DboxOwner<T>>::exists(dbox_id), "Dbox does not exist");	
			ensure!(Some(sender.clone()) == <DboxOwner<T>>::get(dbox_id), "The owner of the dbox is not the sender");

			let mut dbox = Self::get_dbox_by_id(dbox_id).unwrap();
			ensure!(dbox.status == DboxStatus::Active, "The status of dbox should be active");

			ensure!(dbox.value >= Self::dbox_unit_price(), "Not enough money");

			// Create another new dbox with money in the old dbox
			let _ = Self::do_create_dbox(&sender, &None, false)?;
			dbox.value = dbox.value.saturating_sub(Self::dbox_unit_price());
			// Save status
			<AllDboxesArray<T>>::insert(dbox.create_position, &dbox);

			Ok(())
		}

		fn on_finalize(n: T::BlockNumber) {
			let mut ops:i32 = 1000;
			// Update game status
			if GameStatus::get() == Status::Running {
				let mut timeout = Self::timeout();
				timeout = timeout.saturating_sub(10); // TODO: use config value
				if timeout == 0 {
					Self::begin_settling();
				}
				Timeout::mutate(|n| *n = timeout);
			}
			// TODO: print debug info
			// Drain pending bonus
			loop {
				if ops <= 0 {
					break;
				}

				if !Self::drain_bonus(&mut ops) {
					break;
				}
			}

			// Send money to latest boxes
			if GameStatus::get() == Status::Settling {
				loop {
					if ops <= 0 {
						break;
					}

					if !Self::release_prize(&mut ops) {
						Self::reset();
						break;
					}
				}
			}  
        }
	}
}

impl <T: Trait> Module<T> {
	fn check_inviting(invitor_account: &Option<T::AccountId>, invitee: &T::AccountId) -> Result {
		if let Some(invitor) = invitor_account {
			// Make sure invitor is from the owner of previous creatd dbox
			ensure!(<AllPlayers<T>>::exists(invitor), "Invitor should be the player");
			let invitor_player = Self::player(invitor);
			ensure!(invitor_player.status == PlayerStatus::Active, "Invitor should be active player");
			
			// Make sure invitee does not exists
			ensure!(!<AllPlayers<T>>::exists(invitee), "Invitee sh be new player");
			// TODO: add blacklist
		}
		
		Ok(())
	}

	fn ensure_status(status_vec: Vec<Status>) -> Result {
		let current_status = GameStatus::get();
		ensure!(status_vec.iter().any(|status| current_status == *status), "Status is not ready");
		Ok(())
	}

	fn is_staled_dbox(dbox: &DboxOf<T>) -> bool {
		let round_start_dbox = Self::round_start_dbox();
		dbox.create_position < round_start_dbox
	}

	fn check_insert(from: &T::AccountId, dbox_id: &T::Hash, new_dbox: &DboxOf<T>) -> Result {
		ensure!(!<DboxOwner<T>>::exists(dbox_id), "Dbox already exists");

		let owned_dbox_count = Self::owned_dbox_count(from);
		let new_owned_dbox_count = owned_dbox_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to account balance")?;

		let all_dboxes_count = Self::all_dboxes_count();
		let new_all_dboxes_count = all_dboxes_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to total supply")?;

		// limit checking
		let all_active_dboxes_count = Self::all_active_dboxes_count();
		let new_all_active_dboxes_count = all_active_dboxes_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to total active dboxes")?;
		ensure!(new_all_active_dboxes_count < Self::max_active_dboxes_count(), "Exceed max active dboxes limitation");

		Ok(())
	}

	/// Get dbox by hash id
	fn get_dbox_by_id(dbox_id: T::Hash) -> Option<DboxOf<T>> {
		// FIXME: what if the dbox does not exists? 
		let index = <AllDboxesIndex<T>>::get(dbox_id);
		Some(<AllDboxesArray<T>>::get(index))
	}

	/// Insert the new player if does not exist
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
			T::Currency::transfer(&Self::cashier_account(), &invitor_account, commission_amount)?;
			// Update invitor's commission balance
			Self::add_commission(&invitor_account, commission_amount);
		}

		Ok(())
	}

	// Delete balance of priviledged account
	fn del_balance(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
		let balance = <Ledger<T>>::get(account);
		let new_balance = balance.saturating_sub(amount); // FIXME: check saturating_sub
		<Ledger<T>>::insert(account, new_balance);	
		Ok(())
	}

	// Add the bonus 
	fn add_bonus(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
		let mut player = Self::player(account);
		player.total_bonus = player.total_bonus.saturating_add(amount);
		<AllPlayers<T>>::insert(account, player);

		Ok(())
	}

	// Add the prize 
	fn add_prize(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
		let mut player = Self::player(account);
		player.total_prize = player.total_prize.saturating_add(amount);
		<AllPlayers<T>>::insert(account, player);

		Ok(())
	}

	// Add the commission 
	fn add_commission(account: &T::AccountId, amount: BalanceOf<T>) -> Result {
		let mut player = Self::player(account);
		player.total_commission = player.total_commission.saturating_add(amount);
		<AllPlayers<T>>::insert(account, player);

		Ok(())
	}

	// Insert the new dbox
	fn insert_dbox(from: &T::AccountId, dbox_id: T::Hash, new_dbox: &DboxOf<T>) -> Result {
		let _ = Self::check_insert(from, &dbox_id, &new_dbox)?; 

		let owned_dbox_count = Self::owned_dbox_count(from);
		let new_owned_dbox_count = owned_dbox_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to account balance")?;

		let all_dboxes_count = Self::all_dboxes_count();
		let new_all_dboxes_count = all_dboxes_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to total supply")?;

		// Sanity checking
		let all_active_dboxes_count = Self::all_active_dboxes_count();
		let new_all_active_dboxes_count = all_active_dboxes_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to total active dboxes")?;

		<DboxOwner<T>>::insert(dbox_id, from);
		AllActiveDboxesCount::put(new_all_active_dboxes_count);

		<AllDboxesArray<T>>::insert(all_dboxes_count, new_dbox);
		AllDboxesCount::put(new_all_dboxes_count);
		<AllDboxesIndex<T>>::insert(dbox_id, all_dboxes_count);

		<OwnedDboxesArray<T>>::insert((from.clone(), owned_dbox_count), dbox_id);
        <OwnedDboxesCount<T>>::insert(from, new_owned_dbox_count);
        <OwnedDboxesIndex<T>>::insert(dbox_id, owned_dbox_count);

        Self::deposit_event(RawEvent::DboxCreated(from.clone(), dbox_id));

		Ok(())
	}

	/// Send pending bonus
	fn send_pending_bonus(dbox: &mut DboxOf<T>) -> Result {
		let mut prev_dbox = Self::dbox_by_index(dbox.bonus_position);
		if prev_dbox.status == DboxStatus::Active || prev_dbox.status == DboxStatus::Opening {
			prev_dbox.value += dbox.bonus_per_dbox;
			// FIXME
			if prev_dbox.status == DboxStatus::Opening && prev_dbox.open_position == dbox.create_position {
				Self::do_open_dbox(&mut prev_dbox, true);
			} 
			<AllDboxesArray<T>>::insert(prev_dbox.create_position, prev_dbox);
		}

		dbox.bonus_position += 1; // Move forward
		Ok(())
	}

	fn has_pending_bonus(dbox: &DboxOf<T>) -> bool {
		if Self::is_staled_dbox(dbox) {
			return false;
		}
		if Self::bonus_dbox() == Self::all_dboxes_count() {
			return false;
		}
		true
	}

	// Get pending bonus and flag to indicate if player will get double prize
	fn get_pending_bonus(dbox: &DboxOf<T>) -> (bool, bool) {
		let status = GameStatus::get();
		
		if Self::is_staled_dbox(dbox) {
			return (false, false);
		}

		let mut double = true;
		if status == Status::Settling {
			double = false;
		}

		if Self::bonus_dbox() == Self::all_dboxes_count() {
			return (false, double);
		}

		(true, double)
	}

	/// Settle money for the opened dbox
	fn do_open_dbox(dbox: &mut DboxOf<T>, double: bool) -> Result {
		// FIXME: we do not care if transfer is ok or not
		if let Some(player) = Self::owner_of(dbox.id) {
			if !dbox.value.is_zero() {
				let amount = if double {
					dbox.value + dbox.value
				} else {
					dbox.value 
				};
				
				let _ = T::Currency::transfer(&Self::cashier_account(), &player, amount);
				Self::add_bonus(&player, amount);

				if double {
					Self::del_balance(&Self::pool_account(), dbox.value);
				}
			}
		}

		dbox.status = DboxStatus::Opened;
		dbox.value = Zero::zero();
		
		Ok(())
	}

	/// Called when box operation occurs 
	fn on_box_operation(player: &T::AccountId, dbox: &DboxOf<T>) -> Result {
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
		timeout = timeout.max(T::Expiration::get()); // FIXME:
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
		if !<LatestDboxes<T>>::exists(last_dbox_index-1) {
			return None;
		}

		Some(<LatestDboxes<T>>::get(last_dbox_index-1).0)
	}

	/// Reset lastest boxex information
	fn reset_latest_dboxes() -> Result {
		let latest_dboxes_count = Self::latest_dboxes_count();
		let last_dbox_index = Self::last_dbox_index();	

		for i in 1..=latest_dboxes_count {
			<LatestDboxes<T>>::remove(last_dbox_index-i);
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

		Ok(())
	}

	// Drain bonus
	fn drain_bonus(ops: &mut i32) -> bool {
		let bonus_dbox = Self::bonus_dbox();
		if bonus_dbox >= Self::all_dboxes_count() {
			return false;
		}
		
		let mut dbox = Self::dbox_by_index(bonus_dbox);
		// FIXME: 
		if dbox.bonus_per_dbox.is_zero() {
			// update bonus position
			BonusDbox::put(bonus_dbox+1); 
			*ops -= 1;
			return true;
		}

		loop {
			if dbox.bonus_position >= dbox.create_position {
				// update bonus position
				BonusDbox::put(bonus_dbox+1);
				*ops -= 1;
				break;
			}
			// send bonus 
			Self::send_pending_bonus(&mut dbox);
			*ops -= 2;
			// TODO: use config value
			if *ops <= 0 {
				break;
			}
		}
		// update dbox
		<AllDboxesArray<T>>::insert(dbox.create_position, dbox);
		
		true
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
					let _ = T::Currency::transfer(&Self::cashier_account(), &player, last_player_prize);
					let _ = Self::add_prize(&player, &last_player_prize);
					// Reset last player account
				}
			} 
			*ops -= 5;
			return false;
		}

		let i = last_dbox_index - (latest_dboxes_count - released_dboxes_count);
		let (player, dbox_pos) = <LatestDboxes<T>>::get(i); 

		// Share the last prize
		let average_prize = Self::average_prize();
		if !average_prize.is_zero() {
			// FIXME: we don't care if the transfer is ok or not
			let _ = T::Currency::transfer(&Self::cashier_account(), &player, &average_prize);
			let _ = Self::add_prize(&player, &average_prize);
		}
		
		ReleasedDboxesCount::mutate(|n| *n += 1);
		*ops -= 3;

		true
	}

	/// Reset the game and start again, note that all bonus transfer should be finished before reset
	fn reset() -> Result {
		// reset round positions
		let all_dboxes_count = Self::all_dboxes_count();

		RoundStartDbox::put(all_dboxes_count);
		BonusDbox::put(all_dboxes_count);
		AllActiveDboxesCount::put(0);

		// FIXME: Reset balances of proxy accounts
		let accounts: Vec<T::AccountId> = vec![Self::cashier_account(), Self::reserve_account(),
			Self::pool_account(), Self::last_player_account()];

		for account in accounts.iter() {
			let balance = <BalanceOf<T>>::zero();
			<Ledger<T>>::insert(account, balance);
		}

		// Reset lastest dboxes 
		Self::reset_latest_dboxes();

		MaxActiveDboxesCount::put(Self::max_preset_active_dboxes_count());

		// Reset status and timeout value
		GameStatus::put(Status::Running);
		Timeout::put(T::Expiration::get());
		Ok(())
	}
}

decl_event!(
	pub enum Event<T> where 
	Hash = <T as system::Trait>::Hash,
	BlockNumber = <T as system::Trait>::BlockNumber,
	AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
		DboxCreated(AccountId, Hash),
		DboxOpened(AccountId, Hash),
		GameStarted(BlockNumber),
		GameStopped(BlockNumber),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, assert_err, parameter_types};
	use sr_primitives::{traits::{ConvertInto, BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}

	type Balance = u128;

	parameter_types! {
		pub const ExistentialDeposit: u128 = 500;
		pub const TransferFee: u128 = 0;
		pub const CreationFee: u128 = 0;
		pub const TransactionBaseFee: u128 = 0;
		pub const TransactionByteFee: u128 = 1;
	}

	impl balances::Trait for Test {
		type Balance = Balance;
		type OnFreeBalanceZero = ();
		type OnNewAccount = ();
		type Event = ();
		type TransactionPayment = ();
		type DustRemoval = ();
		type TransferPayment = ();

		type ExistentialDeposit = ExistentialDeposit;
		type TransferFee = TransferFee;
		type CreationFee = CreationFee;
		type TransactionBaseFee = TransactionBaseFee;
		type TransactionByteFee = TransactionByteFee;
		type WeightToFee = ConvertInto;
	}

	parameter_types! {
		pub const ExpirationValue: u32 = 12 * 3600; // 12 hours
		pub const MaxLatestValue: u64 = 10;
		pub const MinUnitPrice: Balance = 0; // FIXME: 
		pub const MaxUnitPrice: Balance = 3500000000; // FIXME: 
		pub const DboxRatio: u32 = 35;
		pub const ReserveRatio: u32 = 35;
		pub const PoolRatio: u32 = 10;
		pub const LastPlayerRatio: u32 = 5;
		pub const TeamRatio: u32 = 5;
		pub const OperatorRatio: u32 = 5;
		pub const InvitorRatio: u32 = 5;
	}

	impl Trait for Test {
		type Event = ();
		type Expiration = ExpirationValue;
		type MaxLatest = MaxLatestValue;
		type MinUnitPrice = MinUnitPrice;
		type MaxUnitPrice = MaxUnitPrice;
		type DboxRatio = DboxRatio;
		type ReserveRatio = ReserveRatio;
		type PoolRatio = PoolRatio;
		type LastPlayerRatio = LastPlayerRatio;
		type TeamRatio = TeamRatio;
		type OperatorRatio = OperatorRatio;
		type InvitorRatio = InvitorRatio;
		type Currency = Balances;
	}

	type Balances = balances::Module<Test>;
	type PandoraModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		balances::GenesisConfig::<Test> {
			balances: vec![
				(111, 100_000),
				(555, 500_000),
				(666, 600_000),
				(888, 100_000), // ME
			],
			vesting: vec![],
		}.assimilate_storage(&mut t).unwrap();

		GenesisConfig::<Test> {
			admin_account: 666,	
			cashier_account:111,
			reserve_account: 222,
			pool_account: 333,
			last_player_account:444,
			team_account: 555,
			operator_account: 777,
			
		}.assimilate_storage(&mut t).unwrap();

		t.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(PandoraModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(PandoraModule::something(), Some(42));
		});
	}

	#[test]
	fn it_works_for_init() {
		with_externalities(&mut new_test_ext(), || {
			// Call init
			assert_eq!(PandoraModule::game_status(), Status::None);
			assert_ok!(PandoraModule::init(Origin::signed(666), 100));
			assert_eq!(PandoraModule::game_status(), Status::Inited);
			assert_eq!(PandoraModule::dbox_unit_price(), 100);
		})
	}

	#[test]
	fn it_works_for_creating_dbox() {
		with_externalities(&mut new_test_ext(), || {
			// TODO:  Call create
			assert_ok!(PandoraModule::init(Origin::signed(666), 100));
			assert_ok!(PandoraModule::set_status(Origin::signed(666), Status::Running));
			// Create a dbox
			assert_ok!(PandoraModule::create_dbox(Origin::signed(888), None));
			assert_eq!(PandoraModule::all_dboxes_count(), 1);
			assert_eq!(PandoraModule::all_active_dboxes_count(), 1);
			assert_eq!(Balances::free_balance(&888), 99_900);

			assert_eq!(PandoraModule::balance(&222), 35);
			assert_eq!(PandoraModule::balance(&555), 5);

			// Should error for not enough fund
			assert_err!(PandoraModule::create_dbox(Origin::signed(123), None), "balance too low to send value");
			assert_eq!(PandoraModule::all_dboxes_count(), 1);

		})
	}
}
