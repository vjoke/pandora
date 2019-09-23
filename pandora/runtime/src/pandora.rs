/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result,
	traits::{Get, Currency, ReservableCurrency}, ensure
};
use system::ensure_signed;
use codec::{Encode, Decode};
use sr_primitives::traits::{Hash, Zero};


#[derive(Encode, Decode, Clone, Debug, PartialEq)]
pub enum Status {
	None,
	Inited,
	Running,
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
	/// The dbox is closed, seems unnecessary
	Closed,
}

impl Default for DboxStatus {
	fn default() -> Self {
		DboxStatus::None
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Dbox<Hash, Balance> {
	id: Hash,
	value: Balance,
	version: u64,
	status: DboxStatus,
	// TODO: add more fields
}

/// The module's configuration trait.
pub trait Trait: balances::Trait {
	// TODO: Add other types and constants required configure this module.
	/// Define the expiration in seconds for one round of game
	type Expiration: Get<u32>; 

	/// Define min unit price for dbox
	type MinUnitPrice: Get<BalanceOf<Self>>;
	
	/// Define max unit price of dbox
	type MaxUnitPrice: Get<BalanceOf<Self>>;
	
	/// Define ratios for different parts
	type BoxRatio: Get<u32>;
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
		
		GameStatus get(game_status): Status;

		DboxUnitPrice get(dbox_unit_price): BalanceOf<T>;

		// All the dboxes
		Dboxes get(dbox): map T::Hash => Dbox<T::Hash, T::Balance>;
		DboxOwner get(owner_of): map T::Hash => Option<T::AccountId>;

		AllDboxesArray get(dbox_by_index): map u64 => T::Hash;
		AllDboxesCount get(all_dboxes_count): u64;
		AllDboxesIndex: map T::Hash => u64;

		OwnedDboxesArray get(dbox_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
		OwnedDboxesCount get(owned_dbox_count): map T::AccountId => u64;
		OwnedDboxesIndex: map T::Hash => u64;

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

		const MinUnitPrice: BalanceOf<T> = T::MinUnitPrice::get();
		const MaxUnitPrice: BalanceOf<T> = T::MaxUnitPrice::get();
		// Ratios
		const BoxRatio: u32 = T::BoxRatio::get();
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

		pub fn init(origin, dbox_unit_price: BalanceOf<T>) -> Result {
			// Check signature
			let sender = ensure_signed(origin)?;
			// Check priviledge
			ensure!(sender == Self::admin_account(), "Not authorized");
			ensure!(!GameStatus::exists(), "Already inited");
			// FIXME: Check unit price
			ensure!(dbox_unit_price > T::MinUnitPrice::get(), "Unit price is too low");
			ensure!(dbox_unit_price < T::MaxUnitPrice::get(), "Unit price is too high");
			
			GameStatus::put(Status::Inited);
			<DboxUnitPrice<T>>::put(dbox_unit_price);

			Ok(())
			
		}

		// Create a dbox
		pub fn create_dbox(origin) -> Result {
			let sender = ensure_signed(origin)?;
			// Check if the balance is enough to create a dbox
			// TODO: Check if sender is specific accounts
			let nonce = Nonce::get();
			let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
				.using_encoded(<T as system::Trait>::Hashing::hash);

			let mut new_dbox = Dbox {
				id: random_hash,
				value: Zero::zero(),
				version: 0,
				status: DboxStatus::Active,
			};

			let _ = Self::check_insert(&sender, &random_hash, &new_dbox)?;
			// Check if we can insert dbox without error
			let _ = T::Currency::transfer(&sender, &Self::cashier_account(), Self::dbox_unit_price())?;
			Self::insert(sender, random_hash, new_dbox);

			Nonce::mutate(|n| *n += 1);

			Ok(())
		}
	}
}

impl <T: Trait> Module<T> {
	fn check_insert(from: &T::AccountId, dbox_id: &T::Hash, new_dbox: &Dbox<T::Hash, T::Balance>) -> Result {
		ensure!(!<DboxOwner<T>>::exists(dbox_id), "Dbox already exists");

		let owned_dbox_count = Self::owned_dbox_count(from);
		let new_owned_dbox_count = owned_dbox_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to account balance")?;

		let all_dboxes_count = Self::all_dboxes_count();
		let new_all_dboxes_count = all_dboxes_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to total supply")?;

		Ok(())
	}

	fn insert(from: T::AccountId, dbox_id: T::Hash, new_dbox: Dbox<T::Hash, T::Balance>) -> Result {
		let _ = Self::check_insert(&from, &dbox_id, &new_dbox)?; 

		let owned_dbox_count = Self::owned_dbox_count(&from);
		let new_owned_dbox_count = owned_dbox_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to account balance")?;

		let all_dboxes_count = Self::all_dboxes_count();
		let new_all_dboxes_count = all_dboxes_count.checked_add(1)
			.ok_or("Overflow adding a new dbox to total supply")?;

		<Dboxes<T>>::insert(dbox_id, new_dbox);
		<DboxOwner<T>>::insert(dbox_id, &from);

		<AllDboxesArray<T>>::insert(all_dboxes_count, dbox_id);
		AllDboxesCount::put(new_all_dboxes_count);
		<AllDboxesIndex<T>>::insert(dbox_id, all_dboxes_count);

		<OwnedDboxesArray<T>>::insert((from.clone(), owned_dbox_count), dbox_id);
        <OwnedDboxesCount<T>>::insert(&from, new_owned_dbox_count);
        <OwnedDboxesIndex<T>>::insert(dbox_id, owned_dbox_count);

        Self::deposit_event(RawEvent::DboxCreated(from, dbox_id));

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
	use support::{impl_outer_origin, assert_ok, parameter_types};
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
		pub const MinUnitPrice: Balance = 0; // FIXME: 
		pub const MaxUnitPrice: Balance = 3500000000; // FIXME: 
		pub const BoxRatio: u32 = 35;
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
		type MinUnitPrice = MinUnitPrice;
		type MaxUnitPrice = MaxUnitPrice;
		type BoxRatio = BoxRatio;
		type ReserveRatio = ReserveRatio;
		type PoolRatio = PoolRatio;
		type LastPlayerRatio = LastPlayerRatio;
		type TeamRatio = TeamRatio;
		type OperatorRatio = OperatorRatio;
		type InvitorRatio = InvitorRatio;
	}

	type PandoraModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		balances::GenesisConfig::<Test> {
			balances: vec![
				(5, 500_000),
				(6, 600_000),
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
		})
	}

}
