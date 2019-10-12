/// tests for this module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::pandora::*;

    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use sr_primitives::weights::Weight;
    use sr_primitives::Perbill;
    use sr_primitives::{
        testing::Header,
        traits::{BlakeTwo256, ConvertInto, IdentityLookup, OnFinalize},
    };
    use support::{assert_err, assert_ok, impl_outer_origin, parameter_types};

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
        pub const ExpirationValue: u32 = 50;
        pub const MaxLatestValue: u64 = 5;
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
    type Pandora = Module<Test>;

    // Define previledged acounts
    const ADMIN_ACCOUT: u64 = 10000;
    const CASHIER_ACCOUNT: u64 = 10001;
    const RESERVE_ACCOUNT: u64 = 10002;
    const POOL_ACCOUNT: u64 = 10003;
    const LAST_PLAYER_ACCOUNT: u64 = 10004;
    const TEAM_ACCOUNT: u64 = 10005;
    const OPERATOR_ACCOUNT: u64 = 10006;

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
                (ADMIN_ACCOUT, 600_000),
                (CASHIER_ACCOUNT, 100_000),
                (TEAM_ACCOUNT, 500_000),
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
        // Add config for pandora
        GenesisConfig::<Test> {
            admin_account: ADMIN_ACCOUT,
            cashier_account: CASHIER_ACCOUNT,
            reserve_account: RESERVE_ACCOUNT,
            pool_account: POOL_ACCOUNT,
            last_player_account: LAST_PLAYER_ACCOUNT,
            team_account: TEAM_ACCOUNT,
            operator_account: OPERATOR_ACCOUNT,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }

    #[test]
    fn it_works_for_init() {
        with_externalities(&mut new_test_ext(), || {
            // Call init
            assert_eq!(Pandora::game_status(), Status::None);
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_eq!(Pandora::dbox_unit_price(), 100);
        })
    }

    #[test]
    fn it_works_for_creating_dbox() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Create a dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_eq!(Pandora::all_dboxes_count(), 1);
            assert_eq!(Pandora::all_active_dboxes_count(), 1);
            assert_eq!(Balances::free_balance(&RAY), 99_999_900);

            // Check if money is split properly
            assert_eq!(Pandora::balance(&RESERVE_ACCOUNT), 35);
            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 10);
            assert_eq!(Pandora::balance(&LAST_PLAYER_ACCOUNT), 5);
            assert_eq!(Pandora::balance(&TEAM_ACCOUNT), 5);
            assert_eq!(Pandora::balance(&OPERATOR_ACCOUNT), 5);

            // Should error for not enough fund
            assert_err!(
                Pandora::create_dbox_with_invitor(Origin::signed(123), None),
                "balance too low to send value"
            );
            assert_eq!(Pandora::all_dboxes_count(), 1);
            // Should fail to create dbox with system accounts
            assert_err!(Pandora::create_dbox_with_invitor(Origin::signed(TEAM_ACCOUNT), None), "System account is not allowed");
        })
    }

    #[test]
    fn it_works_for_opening_dbox_sync() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Create dboxes
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            <Pandora as OnFinalize<u64>>::on_finalize(1);

            let dbox = Pandora::dbox_by_index(0);
            let (has_pending, double) = Pandora::get_pending_bonus(&dbox);
            assert_eq!(has_pending, false);
            assert_eq!(double, true);

            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));
            let dbox = Pandora::dbox_by_index(0);
            assert_eq!(dbox.status, DboxStatus::Opened);

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, (35 + 35/2) * 2);

            let dbox = Pandora::dbox_by_index(2);
            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));
            let dbox = Pandora::dbox_by_index(2);
            assert_eq!(dbox.status, DboxStatus::Opened);

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, (35 + 35/2) * 2);

            let dbox = Pandora::dbox_by_index(1);
            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));
            let dbox = Pandora::dbox_by_index(1);
            assert_eq!(dbox.status, DboxStatus::Opened);

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, (35 + 35/2) * 2 + 35/2*2);
        })
    }

    #[test]
    fn it_works_for_opening_dbox_async() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Tries to open non-existed dbox
            assert_err!(Pandora::open_dbox(Origin::signed(RAY), H256::random()), "Dbox does not exist");
            // Create dboxes
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));

            let dbox = Pandora::dbox_by_index(0);
            println!("dbox id={}", dbox.id);
            let (has_pending, double) = Pandora::get_pending_bonus(&dbox);
            assert_eq!(has_pending, true);
            assert_eq!(double, true);

            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));
            let dbox = Pandora::dbox_by_index(0);
            assert_eq!(dbox.status, DboxStatus::Opening);

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, 0);

            for i in 1..6 {
                let bonus_dbox = Pandora::bonus_dbox();
                let status = Pandora::game_status();
                let dbox = Pandora::dbox_by_index(2); 
                println!("bonus_dbox = {} status = {} dbox.bonus_position = {}", bonus_dbox, status as u32, dbox.bonus_position);
                <Pandora as OnFinalize<u64>>::on_finalize(i);
            }
            assert_eq!(Pandora::round_start_dbox(), 3);
            let dbox = Pandora::dbox_by_index(0);
            assert_eq!(dbox.status, DboxStatus::Opened);

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, (35 + 35/2)*2);

            let dbox = Pandora::dbox_by_index(0);
            println!("dbox id={}", dbox.id);
            assert_err!(Pandora::open_dbox(Origin::signed(RAY), dbox.id), "The status of dbox should be active");
            
            let dbox = Pandora::dbox_by_index(1);
            println!("dbox id={}", dbox.id);
            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, 104 + 35/2);
            // 3th dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));

            let dbox = Pandora::dbox_by_index(2);
            println!("dbox id={}", dbox.id);
            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));

            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, 121);

            <Pandora as OnFinalize<u64>>::on_finalize(10);
            assert_eq!(Pandora::all_opening_doxes_count(), 0);
            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, 121); 
            // 4th dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), Some(RAY)));
            assert_err!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), Some(RAY)), "Invitee should be a new player");
            // 5th dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), None));

            let dbox = Pandora::dbox_by_index(4);
            assert_ok!(Pandora::open_dbox(Origin::signed(BOB), dbox.id));
            let dbox = Pandora::dbox_by_index(4);
            assert_eq!(dbox.status, DboxStatus::Opening);
            
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), None));
            <Pandora as OnFinalize<u64>>::on_finalize(11); 

            let dbox = Pandora::dbox_by_index(4);
            assert_eq!(dbox.status, DboxStatus::Opened);
        })
    }

    #[test]
    fn it_works_for_pending_bonus() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Ray creates a dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_eq!(Pandora::all_dboxes_count(), 1);
            assert_eq!(Pandora::all_active_dboxes_count(), 1);

            assert_eq!(Pandora::all_dboxes_count(), 1);
            assert_eq!(Pandora::bonus_dbox(), 0);
            let dbox = Pandora::dbox_by_index(0);
            assert_eq!(dbox.create_position, 0);
            assert_eq!(dbox.bonus_position, 0);
            // Finalize a block
            <Pandora as OnFinalize<u64>>::on_finalize(1);
            assert_eq!(Pandora::timeout(), 40);
            assert_eq!(Pandora::bonus_dbox(), 1);

            // Alice creates a dbox
            assert_ok!(Pandora::create_dbox_with_invitor(
                Origin::signed(ALICE),
                None
            ));
            assert_eq!(Pandora::all_dboxes_count(), 2);
            assert_eq!(Pandora::all_active_dboxes_count(), 2);

            let dbox = Pandora::dbox_by_index(1);
            assert_eq!(dbox.create_position, 1);
            assert_eq!(dbox.bonus_position, 0);

            assert_eq!(Pandora::timeout(), 50);
            // Finalize a block
            <Pandora as OnFinalize<u64>>::on_finalize(2);
            assert_eq!(Pandora::timeout(), 40);
            let dbox = Pandora::dbox_by_index(1);
            assert_eq!(dbox.bonus_position, 1);

            let dbox = Pandora::dbox_by_index(0);
            assert_eq!(dbox.value, 35);

            // Ray open the dbox, will get twice of the box value
            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Balances::free_balance(&RAY), 99_999_900 + 35 * 2);
            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 20);
            assert_eq!(Pandora::average_prize(), 0);
            // Finalize blocks
            for i in 1..5 {
                <Pandora as OnFinalize<u64>>::on_finalize(2 + i);
            }
            assert_eq!(Pandora::timeout(), 10);
            // Finalize blocks
            <Pandora as OnFinalize<u64>>::on_finalize(7);
            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 0);
            assert_eq!(Pandora::average_prize(), 0);
            assert_eq!(
                Balances::free_balance(&RAY),
                99_999_900 + 70 + 20 / 3 * 2 + 5 * 2
            ); // 2 box operations
            assert_eq!(Balances::free_balance(&ALICE), 99_900 + 20 / 3); // 1 box operation
                                                                         // Next round begins
            assert_eq!(Pandora::game_status(), Status::Running);
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::round_start_dbox(), 2);
            assert_eq!(Pandora::all_active_dboxes_count(), 0);
            // Bob creates a new dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), None));
            // Dave creates a new dbox
            assert_ok!(Pandora::create_dbox_with_invitor(
                Origin::signed(DAVE),
                None
            ));
            // Eve creates a new dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(EVE), None));
            // FERDIE creates a new dbox
            assert_ok!(Pandora::create_dbox_with_invitor(
                Origin::signed(FERDIE),
                None
            ));
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::all_dboxes_count(), 6);
            assert_eq!(Pandora::all_active_dboxes_count(), 4);

            // Finalize blocks
            <Pandora as OnFinalize<u64>>::on_finalize(7);
            assert_eq!(Pandora::timeout(), 40);
            // Alice opens a staled dbox
            let dbox = Pandora::dbox_by_index(1);
            assert_eq!(dbox.value, 0);
            assert_eq!(Balances::free_balance(&ALICE), 99_906);

            let dbox = Pandora::dbox_by_index(2);
            assert_eq!(dbox.value, 35 + 35 / 2 + 35 / 3);
            assert_err!(
                Pandora::upgrade_dbox(Origin::signed(RAY), dbox.id),
                "The owner of the dbox is not the sender"
            );
            assert_err!(
                Pandora::upgrade_dbox(Origin::signed(BOB), dbox.id),
                "Not enough money"
            );

            let dbox = Pandora::dbox_by_index(3);
            assert_eq!(dbox.value, 35 / 2 + 35 / 3);

            let dbox = Pandora::dbox_by_index(4);
            assert_eq!(dbox.value, 35 / 3);

            // Finalize blocks
            for i in 1..4 {
                <Pandora as OnFinalize<u64>>::on_finalize(2 + i);
            }
            assert_eq!(Pandora::timeout(), 10);
            assert_eq!(Balances::free_balance(&FERDIE), 99_900);
        })
    }

    #[test]
    fn it_works_for_upgrading_dbox() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Ray creates a dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            let dbox = Pandora::dbox_by_index(0);
            // Bob tries to upgrade a dbox of Ray
            assert_err!(
                Pandora::upgrade_dbox(Origin::signed(BOB), dbox.id),
                "The owner of the dbox is not the sender"
            );
            // Ray tries to upgrade a dbox without enough balance
            assert_err!(
                Pandora::upgrade_dbox(Origin::signed(RAY), dbox.id),
                "Not enough money"
            );
            assert_eq!(dbox.value, 0);
            // Dave creates a dbox and open it at once for 3 times
            for _i in 1..4 {
                assert_ok!(Pandora::create_dbox_with_invitor(
                    Origin::signed(DAVE),
                    None
                ));
                let all_dboxes_count = Pandora::all_dboxes_count();
                let dbox = Pandora::dbox_by_index(all_dboxes_count - 1);
                assert_eq!(dbox.status, DboxStatus::Active);
                assert_ok!(Pandora::open_dbox(Origin::signed(DAVE), dbox.id));
                let dbox = Pandora::dbox_by_index(all_dboxes_count - 1);
                assert_eq!(dbox.status, DboxStatus::Opening);
            }

            for i in 1..5 {
                <Pandora as OnFinalize<u64>>::on_finalize(i);
            }

            assert_eq!(Pandora::timeout(), 10);
            let dbox = Pandora::dbox_by_index(0);
            assert_eq!(dbox.value, 35 * 3);
            // Pause game
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Paused as u8
            ));
            assert_err!(
                Pandora::upgrade_dbox(Origin::signed(RAY), dbox.id),
                "Status is not ready"
            );
            // Run game again
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Upgrade again
            assert_ok!(Pandora::upgrade_dbox(Origin::signed(RAY), dbox.id));
            assert_eq!(Pandora::timeout(), 10 + 30);
            let dbox = Pandora::dbox_by_index(0);
            // Check balance
            assert_eq!(dbox.value, 35 * 3 - 100);
            for i in 1..5 {
                <Pandora as OnFinalize<u64>>::on_finalize(5);
            }
            // 2nd round starts
            assert_eq!(Pandora::round_start_dbox(), 4 + 1); // 4 created dbox with an upgraded dbox
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::all_dboxes_count(), 5);
            let dbox = Pandora::dbox_by_index(4);
            assert_eq!(dbox.value, 0);
            assert_eq!(dbox.create_position, 4);
            assert_eq!(dbox.bonus_position, 4);
            assert_err!(
                Pandora::upgrade_dbox(Origin::signed(RAY), dbox.id),
                "Not enough money"
            );
            assert_eq!(dbox.status, DboxStatus::Active);
        })
    }

    #[test]
    fn it_works_for_staled_dbox() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Ray creates a dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            // Dave creates a dbox and open it at once for 3 times
            for _i in 1..4 {
                assert_ok!(Pandora::create_dbox_with_invitor(
                    Origin::signed(DAVE),
                    None
                ));
                let all_dboxes_count = Pandora::all_dboxes_count();
                let dbox = Pandora::dbox_by_index(all_dboxes_count - 1);
                assert_eq!(dbox.status, DboxStatus::Active);
                assert_ok!(Pandora::open_dbox(Origin::signed(DAVE), dbox.id));
                let dbox = Pandora::dbox_by_index(all_dboxes_count - 1);
                assert_eq!(dbox.status, DboxStatus::Opening);
            }

            for i in 1..6 {
                <Pandora as OnFinalize<u64>>::on_finalize(i);
            }
            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, 0);
            // 2nd round starts
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::round_start_dbox(), 4);
            let dbox = Pandora::dbox_by_index(0);
            // Upgrade the staled dbox
            assert_ok!(Pandora::upgrade_dbox(Origin::signed(RAY), dbox.id));
            let dbox = Pandora::dbox_by_index(0);
            // Check balance
            assert_eq!(dbox.value, 35 * 3 - 100);
            assert_eq!(dbox.status, DboxStatus::Active);
            // Open the staled dbox
            assert_ok!(Pandora::open_dbox(Origin::signed(RAY), dbox.id));
            let player = Pandora::player(RAY);
            assert_eq!(player.total_bonus, 35 * 3 - 100);
        })
    }

    #[test]
    fn it_works_for_draining_bonus() {
       with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100_000));
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // RAY creates dboxes
            let count = 1_000;
            for _i in 0..count {
                assert_ok!(Pandora::create_dbox_with_invitor(
                    Origin::signed(RAY),
                    None
                ));
            }
            assert_eq!(Balances::free_balance(&RAY), 100_000_000 - 100_000 * 1_000);
            assert_eq!(Pandora::all_dboxes_count(), count);
            // Each OnFinalize will do 100 operations by default
            let mut i = 1;
            loop {
                <Pandora as OnFinalize<u64>>::on_finalize(i);
                let bonus_dbox = Pandora::bonus_dbox();
                let status = Pandora::game_status();
                println!("bonus_dbox = {} status = {}", bonus_dbox, status as u32);
                if bonus_dbox == Pandora::all_dboxes_count() {
                    break;
                }
                i += 1;
            }

            assert_eq!(Pandora::game_status(), Status::Running);
            assert_eq!(Pandora::bonus_dbox(), 1000);
       })
    }

    #[test]
    fn it_works_for_inviting() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // Ray creates a dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            let dbox = Pandora::dbox_by_index(0);
            // Bob tries to create a dbox with non-existed player
            assert_err!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), Some(EVE)), "Invitor should be the player");
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), Some(RAY)));
            let player = Pandora::player(RAY);
            assert_eq!(player.total_commission, 5);
            // Bob tries to create a dbox with himself as invitor
            assert_err!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), Some(BOB)), "Invitee should be a new player");
        })
    }

    #[test]
    fn it_works_for_normal_prizes() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // create dboxes
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(DAVE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(NICOLE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));

            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 10*4);
            for i in 1..6 {
               <Pandora as OnFinalize<u64>>::on_finalize(i); 
            }
            // Next round
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::round_count(), 2);
            assert_eq!(Pandora::round_start_dbox(), 4);

            let player = Pandora::player(&BOB);
            assert_eq!(player.total_prize, 10);

            let player = Pandora::player(&DAVE);
            assert_eq!(player.total_prize, 10);
            
            let player = Pandora::player(&RAY);
            assert_eq!(player.total_prize, 10 + 4*5);

            assert_eq!(Pandora::latest_dboxes_count(), 0);
            assert_eq!(Pandora::average_prize(), 0);
        }) 
    }

    #[test]
    fn it_works_for_latest_prizes() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // create dboxes above max latest value
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(DAVE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(EVE), None));
            // The latest MaxLatest players will get prize 
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(NICOLE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(FERDIE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(CHARLIE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(DJANGO), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));

            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 10*8);
            for i in 1..6 {
               <Pandora as OnFinalize<u64>>::on_finalize(i); 
            }
            // Next round
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::round_count(), 2);
            assert_eq!(Pandora::round_start_dbox(), 8);

            let player = Pandora::player(&BOB);
            assert_eq!(player.total_prize, 0);

            let player = Pandora::player(&DAVE);
            assert_eq!(player.total_prize, 0);

            let player = Pandora::player(&EVE);
            assert_eq!(player.total_prize, 0);

            let player = Pandora::player(&NICOLE);
            assert_eq!(player.total_prize, 16);
            
            let player = Pandora::player(&RAY);
            assert_eq!(player.total_prize, 16 + 8*5);

            assert_eq!(Pandora::latest_dboxes_count(), 0);
            assert_eq!(Pandora::average_prize(), 0);
        }) 
    }

    #[test]
    fn it_works_for_duplicate_prizes() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // create dboxes above max latest value
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(BOB), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(DAVE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(EVE), None));
            // The latest MaxLatest players will get prize 
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(NICOLE), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));

            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 10*8);
            for i in 1..6 {
               <Pandora as OnFinalize<u64>>::on_finalize(i); 
            }
            // Next round
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::round_count(), 2);
            assert_eq!(Pandora::round_start_dbox(), 8);

            let player = Pandora::player(&BOB);
            assert_eq!(player.total_prize, 0);

            let player = Pandora::player(&DAVE);
            assert_eq!(player.total_prize, 0);

            let player = Pandora::player(&EVE);
            assert_eq!(player.total_prize, 0);

            let player = Pandora::player(&NICOLE);
            assert_eq!(player.total_prize, 16);
            
            let player = Pandora::player(&RAY);
            assert_eq!(player.total_prize, 16*4 + 8*5);

            assert_eq!(Pandora::latest_dboxes_count(), 0);
            assert_eq!(Pandora::average_prize(), 0);
        }) 
    }

    #[test]
    fn it_works_for_single_player_prizes() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));
            // create a single dbox
            assert_ok!(Pandora::create_dbox_with_invitor(Origin::signed(RAY), None));

            assert_eq!(Pandora::balance(&LAST_PLAYER_ACCOUNT), 5);
            assert_eq!(Pandora::balance(&POOL_ACCOUNT), 10*1);

            for i in 1..6 {
               <Pandora as OnFinalize<u64>>::on_finalize(i); 
            }
            // Next round
            assert_eq!(Pandora::timeout(), 50);
            assert_eq!(Pandora::round_count(), 2);
            assert_eq!(Pandora::round_start_dbox(), 1);
            
            let player = Pandora::player(&RAY);
            assert_eq!(player.total_prize, 10 + 5);

            assert_eq!(Pandora::latest_dboxes_count(), 0);
            assert_eq!(Pandora::average_prize(), 0);
            assert_eq!(Pandora::balance(&LAST_PLAYER_ACCOUNT), 0);
        }) 
    }

    #[test]
    fn it_works_for_setting_status() {
        with_externalities(&mut new_test_ext(), || {
            // Init the game
            assert_ok!(Pandora::init(Origin::signed(ADMIN_ACCOUT), 100));
            assert_eq!(Pandora::game_status(), Status::Inited);
            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ));

            assert_err!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Inited as u8
            ), "Invalid new status");
            
            assert_err!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Running as u8
            ), "New status should be different from current status");

            assert_err!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                100
            ), "Invalid status value"); 

            assert_err!(Pandora::set_status(
                Origin::signed(RAY),
                100
            ), "Not authorized"); 

            assert_ok!(Pandora::set_status(
                Origin::signed(ADMIN_ACCOUT),
                Status::Stopped as u8
            ));
        }) 
    }

}
