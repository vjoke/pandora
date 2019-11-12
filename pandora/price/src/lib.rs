#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use oracle::OracleMixedIn;
use rstd::prelude::*;
use sr_primitives::traits::{Bounded, CheckedAdd, CheckedSub, EnsureOrigin, OnFinalize, Zero};
use support::traits::{
    ChangeMembers, Currency, Get, LockIdentifier, LockableCurrency, ReservableCurrency,
    WithdrawReasons,
};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue,
};
use system::{ensure_root, ensure_signed};

#[cfg(test)]
mod price_test;

type Price = u128;

pub trait Trait: balances::Trait {
    /// Round length
    type RoundLength: Get<Self::BlockNumber>;
    /// Oracle timeout
    type OracleTimeout: Get<Self::BlockNumber>;
    /// Max oracle count
    type MaxOracleCount: Get<u32>;
    /// Oracle Mixed in
    type OracleMixedIn: OracleMixedIn<Self>;
    /// The currency type
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    /// Event
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    // type ReportOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PriceReport<AccountId> {
    reporter: AccountId,
    price: Price,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PendingRequest<Hash, BlockNumber> {
    id: Hash,
    expired_at: BlockNumber,
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Price is reported
        PriceReported(AccountId, Price),
        /// Price is changed
        PriceChanged(Price),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as PriceStorage {
        /// Current price
        CurrentPrice get(current_price): Price;
        /// Price reporters
        PriceReports get(price_reports): Vec<PriceReport<T::AccountId>>;
        /// Job hash array
        PendingRequests get(pending_requests): Vec<PendingRequest<T::Hash, T::BlockNumber>>;
        /// The admin account
        AdminAccount get(admin_account) config(): T::AccountId;
        /// The cashier account
        CashierAccount get(cashier_account) config(): T::AccountId;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;
        const RoundLength: T::BlockNumber = T::RoundLength::get();
        const OracleTimeout: T::BlockNumber = T::OracleTimeout::get();
        const MaxOracleCount: u32 = T::MaxOracleCount::get();
        /// Request price from oracle
        ///
        /// @origin
        /// @oracle the oracle account
        pub fn request_price(origin, oracle: T::AccountId) -> Result {
            let sender = ensure_signed(origin)?;
            ensure!(sender == Self::admin_account(), "Not authorized");
            // TODO:
            let meta = vec![0;32];
            let timeout = T::OracleTimeout::get(); 

            match T::OracleMixedIn::create_request(&Self::cashier_account(), &meta, timeout, &oracle) {
                Ok(hash) => {
                    Self::add_pending_request(hash, Self::block_number() + timeout);
                    Ok(())
                },
                Err(err) => Err(err)
            }
        }

        /// Report price
        ///
        /// @origin
        /// @price  current price
        /// @id the request id
        pub fn report_price(origin, price: Price, id: T::Hash) -> Result {
            // let who = T::ReportOrigin::ensure_origin(origin)?;
            let who = ensure_signed(origin)?;
            T::OracleMixedIn::on_request_fulfilled(&who, id)?;

            Self::add_price(who, price);
            Self::remove_pending_request(id);

            Ok(())
        }

        /// Callback when a block is finalized
        ///
        /// @n  the block number
        fn on_finalize(block_number: T::BlockNumber) {
            let old_price = Self::current_price();
            let mut prices: Vec<Price> = Self::price_reports().iter().map(|x| x.price).collect();
            // Update price
            if prices.len() > 0 {
                let median_price = median(&mut prices);

                if old_price != median_price {
                    CurrentPrice::put(median_price);
                    Self::deposit_event(RawEvent::PriceChanged(median_price));
                }
            }
            // Reset reports if round ends
            if T::BlockNumber::zero() == block_number % T::RoundLength::get() {
                let reports = <Vec<PriceReport<T::AccountId>>>::new();
                <PriceReports<T>>::put(reports);
            }
            // Cancel expired requests for refunding
            Self::cancel_expired_requests(block_number);
        }
    }
}

/// Helper functions
impl<T: Trait> Module<T> {
    /// Add pending request
    ///
    /// @id hash of request
    /// @expired_at expiration block number
    fn add_pending_request(id: T::Hash, expired_at: T::BlockNumber) -> Result {
        let mut requests = Self::pending_requests();
        let request = PendingRequest {
            id: id,
            expired_at: expired_at,
        };

        requests.push(request);
        <PendingRequests<T>>::put(requests);

        Ok(())
    }

    /// Remove pending request
    ///
    /// @id hash of the pending request
    fn remove_pending_request(hash: T::Hash) -> Result {
        let mut requests = Self::pending_requests();
        requests.retain(|x| x.id != hash);

        <PendingRequests<T>>::put(requests);
        Ok(())
    }

    /// Cancel expired requests
    ///
    /// @block_number   current block number
    fn cancel_expired_requests(block_number: T::BlockNumber) -> Result {
        let mut requests = Self::pending_requests();
        requests.retain(|x| {
            if x.expired_at > block_number {
                true
            } else {
                let _ = T::OracleMixedIn::cancel_request(&Self::cashier_account(), x.id);
                false
            }
        });

        <PendingRequests<T>>::put(requests);

        Ok(())
    }

    /// Add price report
    ///
    /// @who    the reporter
    /// @price  the price
    fn add_price(who: T::AccountId, price: Price) -> Result {
        let price_reports = Self::price_reports();
        let mut found = false;
        let mut price_reports: Vec<PriceReport<T::AccountId>> = price_reports
            .into_iter()
            .map(|x| {
                if x.reporter == who {
                    let mut new_report = x;
                    new_report.price = price;
                    found = true;
                    new_report
                } else {
                    x
                }
            })
            .collect();

        if !found {
            price_reports.push(PriceReport {
                reporter: who.clone(),
                price: price,
            });
        }

        <PriceReports<T>>::put(price_reports);
        Self::deposit_event(RawEvent::PriceReported(who, price));

        Ok(())
    }

    /// Get current block number
    fn block_number() -> T::BlockNumber {
        <system::Module<T>>::block_number()
    }
}

/// Calculate median value
///
/// @numbers    the numbers
fn median(numbers: &mut Vec<Price>) -> Price {
    numbers.sort();

    if numbers.len() == 1 {
        return numbers[0];
    }

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as Price
    } else {
        numbers[mid]
    }
}

/// Calculate mean value of prices
///
/// @numbers    the numbers
fn mean(numbers: &Vec<Price>) -> Price {
    let sum: Price = numbers.iter().sum();
    sum as Price / numbers.len() as Price
}
