/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    use rstd::result;
    use oracle::OracleMixedIn;
    use primitives::u32_trait::{_1, _2};
    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use sr_primitives::weights::Weight;
    use sr_primitives::Perbill;
    use sr_primitives::{
        testing::Header,
        traits::{BlakeTwo256, ConvertInto, IdentityLookup, EnsureOrigin, OnFinalize},
    };
    use support::{assert_err, assert_noop, assert_ok, impl_outer_origin, parameter_types};

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
    type AccountId = u64;

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
        pub const RoundLength: u64 = 100;
        pub const OracleTimeout: u64 = 3;
        pub const MaxOracleCount: u32 = 10;
    }

    impl Trait for Test {
        type RoundLength = RoundLength;
        type OracleTimeout = OracleTimeout;
        type MaxOracleCount = MaxOracleCount;
        type OracleMixedIn = Self;
        type Event = ();
        type Currency = Balances;
        // type ReportOrigin = Origin;
    }

    // Mock implementation
    impl<T: Trait> OracleMixedIn<T> for Test {
        /// Create request
        fn create_request(
            from: &T::AccountId,
            meta: &Vec<u8>,
            timeout: T::BlockNumber,
            oracle: &T::AccountId,
        ) -> result::Result<T::Hash, &'static str> {
            Ok(T::Hash::default())
        }
        /// Cancel request
        fn cancel_request(from: &T::AccountId, id: T::Hash) -> Result {
            Ok(())
        }
        /// Called after request is fulfilled
        fn on_request_fulfilled(oracle: &T::AccountId, id: T::Hash) -> Result {
            Ok(())
        }
        /// Predicate if one oracle is valid.
        fn is_valid(who: &T::AccountId) -> bool {
            true
        }
    }

    type Balances = balances::Module<Test>;
    type Price = Module<Test>;
    type System = system::Module<Test>;

    // Define previledged acounts
    const ADMIN_ACCOUNT: u64 = 10000;
    const CASHIER_ACCOUNT: u64 = 10001;

    // Define general player account
    const ALICE: u64 = 100;
    const BOB: u64 = 101;
    const DAVE: u64 = 102;
    const EVE: u64 = 103;
    const FERDIE: u64 = 104;
    const CHARLIE: u64 = 105;
    const DJANGO: u64 = 106;
    const NICOLE: u64 = 107;
    const RAY: u64 = 108;

    // This function basically just builds a genesis storage key/value store according to
    // our desired mockup.
    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        let mut t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        // Add config for balances
        balances::GenesisConfig::<Test> {
            balances: vec![
                (ADMIN_ACCOUNT, 600_000),
                (CASHIER_ACCOUNT, 100_000),
                (ALICE, 100_000),
                (BOB, 100_000),
                (DAVE, 100_000),
                (EVE, 100_000),
                (FERDIE, 100_000),
                (CHARLIE, 100_000),
                (DJANGO, 100_000),
                (NICOLE, 100_000),
                (RAY, 100_000_000),
            ],
            vesting: vec![],
        }
        .assimilate_storage(&mut t)
        .unwrap();
        // Add config for oracle
        GenesisConfig::<Test> {
            admin_account: ADMIN_ACCOUNT,
            cashier_account: CASHIER_ACCOUNT,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }

    #[test]
    fn it_works_for_requesting_price() {
        with_externalities(&mut new_test_ext(), || {
            System::set_block_number(1);
            assert_err!(
                Price::request_price(Origin::signed(RAY), DJANGO),
                "Not authorized"
            );
            // Request for price
            assert_ok!(Price::request_price(Origin::signed(ADMIN_ACCOUNT), DJANGO));
            let request = Price::pending_requests()[0];
            print!("{:#?}", request);
            // Oracle report request
            let price = 5000;
            assert_ok!(Price::report_price(Origin::signed(DJANGO), price, request.id));
            assert_eq!(Price::current_price(), 0);
            <Price as OnFinalize<u64>>::on_finalize(1);
            assert_eq!(Price::current_price(), price);

            let price = 6000;
            let id = H256::random();
            assert_ok!(Price::report_price(Origin::signed(DJANGO), price, id));
            assert_eq!(Price::current_price(), 5000);
            <Price as OnFinalize<u64>>::on_finalize(2);
            assert_eq!(Price::current_price(), price); 

            let price = 7000;
            let id = H256::random();
            assert_ok!(Price::report_price(Origin::signed(DAVE), price, id));

            let price = 8000;
            assert_ok!(Price::report_price(Origin::signed(BOB), price, request.id));

            <Price as OnFinalize<u64>>::on_finalize(2);
            assert_eq!(Price::current_price(), 7000);

            <Price as OnFinalize<u64>>::on_finalize(100);
            assert_eq!(Price::price_reports(), []);
        })
    }
}
