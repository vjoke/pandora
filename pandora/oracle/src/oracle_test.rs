/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    use primitives::u32_trait::{_1, _2};
    use primitives::{Blake2Hasher, H256};
    use runtime_io::TestExternalities;
    use sr_primitives::weights::Weight;
    use sr_primitives::Perbill;
    use sr_primitives::{
        testing::Header,
        traits::{BlakeTwo256, ConvertInto, IdentityLookup, OnFinalize},
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
        type DustRemoval = ();
        type TransferPayment = ();
        type ExistentialDeposit = ExistentialDeposit;
        type TransferFee = TransferFee;
        type CreationFee = CreationFee;
    }

    // type OracleCollective = collective::Instance1;
    // impl collective::Trait<OracleCollective> for Test {
    //     type Origin = Origin;
    //     type Proposal = Call;
    //     type Event = ();
    // }

    // impl collective::Trait for Test {
    //     type Origin = Origin;
    //     type Proposal = Call;
    //     type Event = ();
    // }

    parameter_types! {
        pub const MaxTimeout: u64 = 3;

        pub const OracleFee: Balance = 10;
        pub const MissReportSlash: Balance = 1;
        pub const MinStaking: Balance = 100;

        pub const Count: u16 = 3;

        pub const ReportInterval: u64 = 10;
        pub const ElectionEra: u64 = 10;
        pub const LockedDuration: u64 = 20;
    }

    impl Trait for Test {
        type MaxTimeout = MaxTimeout;
        type OracleFee = OracleFee;
        type MissReportSlash = MissReportSlash;
        type MinStaking = MinStaking;

        // type MaliciousSlashOrigin = collective::EnsureProportionMoreThan<_1, _2, AccountId, OracleCollective>;
        type Count = Count;
        type ReportInteval = ReportInterval;
        type ElectionEra = ElectionEra;
        type LockedDuration = LockedDuration;
        type ChangeMembers = ();
        // type ChangeMembers = OracleMembers;
        type Event = ();
        type Currency = Balances;
    }

    type Balances = balances::Module<Test>;
    type Oracle = Module<Test>;
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
    fn new_test_ext() -> runtime_io::TestExternalities {
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
            cashier_account: CASHIER_ACCOUNT,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }

    #[test]
    fn it_works_for_bonding() {
        new_test_ext().execute_with(|| {
            assert_err!(
                Oracle::bond(Origin::signed(ALICE), 10),
                "Total staked amount is too small"
            );
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 100));
            assert_eq!(Balances::free_balance(&ALICE), 100_000);
            assert_err!(
                Oracle::bond(Origin::signed(ALICE), 100_001),
                "Not enough money"
            );
            assert_err!(
                Oracle::bond(Origin::signed(ALICE), 99_901),
                "Cannot stake more funds than owned"
            );
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 99_900));
            assert_err!(
                Oracle::bond(Origin::signed(ALICE), 1),
                "Cannot stake more funds than owned"
            );

            assert_eq!(Oracle::oracles(), []);
            assert_eq!(Oracle::candidates(), [ALICE]);

            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 100_000);
            assert_eq!(Balances::free_balance(&ALICE), 100_000);
        })
    }

    #[test]
    fn it_works_for_normal_unbonding() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 200));
            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 200);

            assert_noop!(
                Oracle::unbond(Origin::signed(ALICE), 0),
                "The unbond amount is zero or larger than staked funds"
            );

            assert_noop!(
                Oracle::unbond(Origin::signed(ALICE), 201),
                "The unbond amount is zero or larger than staked funds"
            );

            assert_ok!(Oracle::unbond(Origin::signed(ALICE), 10));

            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 200);
            assert_eq!(
                ledger.unbonds[0],
                Unbond {
                    amount: 10,
                    until: 21
                }
            );

            <Oracle as OnFinalize<u64>>::on_finalize(21);

            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 190);
            assert_eq!(ledger.unbonds, []);
        })
    }

    #[test]
    fn it_works_for_unbonding_to_unqualified_member() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 100));
            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 100);

            assert_ok!(Oracle::unbond(Origin::signed(ALICE), 10));

            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 100);
            assert_eq!(
                ledger.unbonds[0],
                Unbond {
                    amount: 100,
                    until: 21
                }
            );

            <Oracle as OnFinalize<u64>>::on_finalize(21);

            let ledger = Oracle::ledger(ALICE);
            assert_eq!(ledger.locked, 0);
            assert_eq!(ledger.unbonds, []);
        })
    }

    #[test]
    fn it_works_for_candidate() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 100));
            let candidates = Oracle::candidates();
            assert_eq!(candidates[0], ALICE);

            assert_ok!(Oracle::bond(Origin::signed(ALICE), 100));
            let candidates = Oracle::candidates();
            assert_eq!(candidates[0], ALICE);

            assert_ok!(Oracle::bond(Origin::signed(BOB), 100));
            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB]);
        })
    }

    #[test]
    fn it_works_for_electing_oracle() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 150));
            assert_ok!(Oracle::bond(Origin::signed(BOB), 200));
            assert_ok!(Oracle::bond(Origin::signed(DAVE), 300));
            assert_ok!(Oracle::bond(Origin::signed(CHARLIE), 100));

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB, DAVE, CHARLIE]);

            <Oracle as OnFinalize<u64>>::on_finalize(10);

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [CHARLIE]);

            let oracles = Oracle::oracles();
            assert_eq!(oracles, [DAVE, BOB, ALICE]); // in reverse order
        })
    }

    #[test]
    fn it_works_for_unqualified_member() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 120));
            assert_ok!(Oracle::bond(Origin::signed(BOB), 200));
            assert_ok!(Oracle::bond(Origin::signed(DAVE), 300));
            assert_ok!(Oracle::bond(Origin::signed(CHARLIE), 100));

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB, DAVE, CHARLIE]);

            <Oracle as OnFinalize<u64>>::on_finalize(10);

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [CHARLIE]);

            let oracles = Oracle::oracles();
            assert_eq!(oracles, [DAVE, BOB, ALICE]); // in reverse order

            assert_ok!(Oracle::unbond(Origin::signed(ALICE), 10));
            let oracles = Oracle::oracles();
            assert_eq!(oracles, [DAVE, BOB, ALICE]);

            assert_ok!(Oracle::unbond(Origin::signed(BOB), 150));
            let oracles = Oracle::oracles();
            assert_eq!(oracles, [DAVE, ALICE]);

            let members = Oracle::unqualified_members();
            assert_eq!(members, [BOB]);

            assert_ok!(Oracle::bond(Origin::signed(BOB), 400));
            let candidates = Oracle::candidates();
            assert_eq!(candidates, [CHARLIE, BOB]);

            <Oracle as OnFinalize<u64>>::on_finalize(20);
            let oracles = Oracle::oracles();
            assert_eq!(oracles, [BOB, DAVE, ALICE]);

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [CHARLIE]);

            let members = Oracle::unqualified_members();
            assert_eq!(members, []);
        })
    }

    #[test]
    fn it_works_for_claiming_reward() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 120));
            assert_ok!(Oracle::bond(Origin::signed(BOB), 200));
            assert_ok!(Oracle::bond(Origin::signed(DAVE), 300));
            assert_ok!(Oracle::bond(Origin::signed(CHARLIE), 100));

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB, DAVE, CHARLIE]);

            <Oracle as OnFinalize<u64>>::on_finalize(10);

            assert_noop!(
                Oracle::claim_reward(Origin::signed(ALICE), 0),
                "Amount should not be zero"
            );

            assert_noop!(
                Oracle::claim_reward(Origin::signed(ALICE), 10),
                "Exceed withdrawable funds"
            );

            // TODO:
        })
    }

    #[test]
    fn it_works_for_creating_request() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 120));
            assert_ok!(Oracle::bond(Origin::signed(BOB), 200));
            assert_ok!(Oracle::bond(Origin::signed(DAVE), 300));
            assert_ok!(Oracle::bond(Origin::signed(CHARLIE), 100));

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB, DAVE, CHARLIE]);

            <Oracle as OnFinalize<u64>>::on_finalize(10);
            // The length of max meta data should be <= 1024
            let result = Oracle::create_request(&RAY, &vec![1; 1025], 3, &ALICE);
            assert_err!(
                result,
                "The length of meta should be equal or less than 1024"
            );
            // The timeout should be valid
            let result = Oracle::create_request(&RAY, &vec![1; 100], 0, &ALICE);
            assert_err!(result, "Invalid timeout range, should be (0, MaxTimeout]");

            let result = Oracle::create_request(&RAY, &vec![1; 100], 10, &ALICE);
            assert_err!(result, "Invalid timeout range, should be (0, MaxTimeout]");
            // The account requested should be an oracle
            let result = Oracle::create_request(&RAY, &vec![1; 100], 3, &NICOLE);
            assert_err!(result, "Should be a valid oracle");

            let result = Oracle::create_request(&RAY, &vec![1, 2, 3], 3, &ALICE);

            assert_eq!(result.is_ok(), true);
            println!("result id is {}", result.unwrap());
            assert_eq!(Nonce::get(), 1);
        })
    }

    #[test]
    fn it_works_for_cancelling_request() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 120));
            assert_ok!(Oracle::bond(Origin::signed(BOB), 200));
            assert_ok!(Oracle::bond(Origin::signed(DAVE), 300));
            assert_ok!(Oracle::bond(Origin::signed(CHARLIE), 100));

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB, DAVE, CHARLIE]);

            <Oracle as OnFinalize<u64>>::on_finalize(10);
            // Cancel non-existed request should fail
            assert_err!(
                Oracle::cancel_request(&RAY, H256::random()),
                "Job does not exist"
            );
            // Create a normal request
            let result = Oracle::create_request(&RAY, &vec![1, 2, 3], 3, &ALICE);
            // Result should be ok
            assert_eq!(result.is_ok(), true);
            println!("result id is {}", result.unwrap());
            let info = Oracle::oracle_info(ALICE);
            assert_eq!(info.total_jobs, 1);
            // Cancelling other's request should fail
            assert_err!(
                Oracle::cancel_request(&DAVE, result.unwrap()),
                "Not authorized"
            );

            // Normal cancelling should be ok
            System::set_block_number(2);
            assert_err!(
                Oracle::cancel_request(&RAY, result.unwrap()),
                "Job is not expired"
            );

            // Cancelling expired request should be ok
            System::set_block_number(4);
            assert_ok!(Oracle::cancel_request(&RAY, result.unwrap()));

            let info = Oracle::oracle_info(RAY);
            assert_eq!(info.total_jobs, 0);
        })
    }

    #[test]
    fn it_works_for_fulfilling_request() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);
            assert_ok!(Oracle::bond(Origin::signed(ALICE), 120));
            assert_ok!(Oracle::bond(Origin::signed(BOB), 200));
            assert_ok!(Oracle::bond(Origin::signed(DAVE), 300));
            assert_ok!(Oracle::bond(Origin::signed(CHARLIE), 100));

            let candidates = Oracle::candidates();
            assert_eq!(candidates, [ALICE, BOB, DAVE, CHARLIE]);

            <Oracle as OnFinalize<u64>>::on_finalize(10);
            // Cancel non-existed request should fail
            assert_err!(
                Oracle::cancel_request(&RAY, H256::random()),
                "Job does not exist"
            );
            // Create a normal request
            let result = Oracle::create_request(&RAY, &vec![1, 2, 3], 3, &ALICE);
            // Result should be ok
            assert_eq!(result.is_ok(), true);
            println!("result id is {}", result.unwrap());
            let info = Oracle::oracle_info(ALICE);
            assert_eq!(info.total_jobs, 1);
            assert_eq!(info.total_witnessed_jobs, 0);

            // Fulfilling other's request should fail
            assert_err!(
                Oracle::on_request_fulfilled(&DAVE, result.unwrap()),
                "Not authorized"
            );
            // Fulfilling expired request should fail
            System::set_block_number(4);
            assert_err!(
                Oracle::on_request_fulfilled(&ALICE, result.unwrap()),
                "Job already expired"
            );
            // Normal operation should be ok
            System::set_block_number(2);
            assert_ok!(Oracle::on_request_fulfilled(&ALICE, result.unwrap()));

            let info = Oracle::oracle_info(ALICE);
            assert_eq!(info.total_jobs, 1);
            assert_eq!(info.total_witnessed_jobs, 1);
        })
    }
}
