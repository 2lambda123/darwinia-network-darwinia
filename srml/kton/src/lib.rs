#![cfg_attr(not(feature = "std"), no_std)]

mod imbalance;

#[cfg(test)]
mod tests;

// --- std ---
#[cfg(feature = "std")]
use rstd::borrow::ToOwned;
use rstd::vec::Vec;
// --- external ---
use parity_codec::{Codec, Decode, Encode};
use primitives::traits::{
    Bounded, CheckedAdd, CheckedSub, MaybeSerializeDebug, Member, Saturating, SimpleArithmetic,
    StaticLookup, Zero,
};
use srml_support::{
    decl_event, decl_module, decl_storage,
    dispatch::Result as SResult,
    traits::{
        Currency, ExistenceRequirement, Imbalance, LockIdentifier, LockableCurrency, OnUnbalanced,
        SignedImbalance, UpdateBalanceOutcome, WithdrawReason, WithdrawReasons,
    },
    Parameter, StorageMap, StorageValue,
};
use system::ensure_signed;
// --- custom ---
use imbalance::{NegativeImbalance, PositiveImbalance};

/// Struct to encode the vesting schedule of an individual account.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct VestingSchedule<Balance> {
    /// Locked amount at genesis.
    pub offset: Balance,
    /// Amount that gets unlocked every block from genesis.
    pub per_block: Balance,
}

impl<Balance: Copy + SimpleArithmetic> VestingSchedule<Balance> {
    /// Amount locked at block `n`.
    pub fn locked_at<BlockNumber>(&self, n: BlockNumber) -> Balance
    where
        Balance: From<BlockNumber>,
    {
        if let Some(x) = Balance::from(n).checked_mul(&self.per_block) {
            self.offset.max(x) - x
        } else {
            Balance::zero()
        }
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct BalanceLock<Balance, BlockNumber> {
    pub id: LockIdentifier,
    pub amount: Balance,
    pub until: BlockNumber,
    pub reasons: WithdrawReasons,
}

pub trait Trait: timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type Balance: Codec
        + Copy
        + Default
        + From<Self::BlockNumber>
        + MaybeSerializeDebug
        + Member
        + Parameter
        + SimpleArithmetic;

    // kton
    type OnMinted: OnUnbalanced<PositiveImbalance<Self>>;
    type OnRemoval: OnUnbalanced<NegativeImbalance<Self>>;
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as Trait>::Balance,
    {
        /// Transfer succeeded (from, to, value, fees).
        TokenTransfer(AccountId, AccountId, Balance),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as Kton {
        /// For Currency and LockableCurrency Trait
        /// The total `units issued in the system.
        // like `existential_deposit`, but always set to 0
        pub
            MinimumBalance
            get(minimum_balance):
            T::Balance = 0.into();

        pub
            TotalIssuance
            get(total_issuance)
            build(|config: &GenesisConfig<T>| {
                config
                    .balances
                    .iter()
                    .fold(0.into(), |acc, &(_, n)| acc + n)
            }):
            T::Balance;

        pub
            FreeBalance
            get(free_balance)
            build(|config: &GenesisConfig<T>| config.balances.to_owned()):
            map T::AccountId => T::Balance;

        pub
            ReservedBalance
            get(reserved_balance):
            map T::AccountId => T::Balance;

        pub
            Locks
            get(locks):
            map T::AccountId => Vec<BalanceLock<T::Balance, T::BlockNumber>>;

        pub
            TotalLock
            get(total_lock):
            T::Balance;

        pub
            Vesting
            get(vesting)
            build(|config: &GenesisConfig<T>| {
                config
                    .vesting
                    .iter()
                    .filter_map(|&(ref who, begin, length)| {
                        let begin = <T::Balance as From<T::BlockNumber>>::from(begin);
                        let length = <T::Balance as From<T::BlockNumber>>::from(length);
                        config
                            .balances
                            .iter()
                            .find(|(id, _)| id == who)
                            .map(|&(_, balance)| {
                                // <= begin it should be >= balance
                                // >= (begin + length) it should be <= 0

                                let per_block = balance / length.max(1.into());
                                let offset = begin * per_block + balance;

                                (who.to_owned(), VestingSchedule { offset, per_block })
                            })
                    }).collect::<Vec<_>>()
            }):
            map T::AccountId => Option<VestingSchedule<T::Balance>>;
    }

    add_extra_genesis {
        config(balances): Vec<(T::AccountId, T::Balance)>;
        // begin, length
        config(vesting): Vec<(T::AccountId, T::BlockNumber, T::BlockNumber)>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call
    where
        origin: T::Origin
    {
        fn deposit_event<T>() = default;

        pub fn transfer(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            #[compact] value: T::Balance
        ) {
            let transactor = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;
            <Self as Currency<_>>::transfer(&transactor, &dest, value)?;
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn vesting_balance(who: &T::AccountId) -> T::Balance {
        if let Some(v) = Self::vesting(who) {
            Self::free_balance(who)
                .min(v.locked_at::<T::BlockNumber>(<system::Module<T>>::block_number()))
        } else {
            0.into()
        }
    }

    // PRIVATE MUTABLE
    // NOTE: different from balances module
    fn set_free_balance(who: &T::AccountId, balance: T::Balance) -> UpdateBalanceOutcome {
        // TODO: check the value of balance, but no ensure!(...)
        <FreeBalance<T>>::insert(who, balance);
        UpdateBalanceOutcome::Updated
    }

    fn set_reserved_balance(who: &T::AccountId, balance: T::Balance) -> UpdateBalanceOutcome {
        <ReservedBalance<T>>::insert(who, balance);
        UpdateBalanceOutcome::Updated
    }
}

impl<T: Trait> Currency<T::AccountId> for Module<T> {
    type Balance = T::Balance;
    type PositiveImbalance = PositiveImbalance<T>;
    type NegativeImbalance = NegativeImbalance<T>;

    fn total_balance(who: &T::AccountId) -> Self::Balance {
        Self::free_balance(who) + Self::reserved_balance(who)
    }

    fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
        Self::free_balance(who) >= value
    }

    fn total_issuance() -> Self::Balance {
        Self::total_issuance()
    }

    fn minimum_balance() -> Self::Balance {
        Self::minimum_balance()
    }

    // TODO: ready for hacking
    fn burn(mut amount: Self::Balance) -> Self::PositiveImbalance {
        <TotalIssuance<T>>::mutate(|issued| {
            issued.checked_sub(&amount).unwrap_or_else(|| {
                amount = *issued;
                0.into()
            })
        });

        PositiveImbalance::new(amount)
    }

    // TODO: ready for hacking
    fn issue(mut amount: Self::Balance) -> Self::NegativeImbalance {
        <TotalIssuance<T>>::mutate(|issued| {
            *issued = issued.checked_add(&amount).unwrap_or_else(|| {
                amount = Self::Balance::max_value() - *issued;
                Self::Balance::max_value()
            })
        });

        NegativeImbalance::new(amount)
    }

    fn free_balance(who: &T::AccountId) -> Self::Balance {
        <FreeBalance<T>>::get(who)
    }

    fn ensure_can_withdraw(
        who: &T::AccountId,
        _amount: T::Balance,
        reason: WithdrawReason,
        new_balance: T::Balance,
    ) -> SResult {
        match reason {
            WithdrawReason::Reserve | WithdrawReason::Transfer => {
                if Self::vesting_balance(who) > new_balance {
                    return Err("vesting balance too high to send value");
                }
            }
            _ => (),
        }

        let locks = Self::locks(who);

        if !locks.is_empty() {
            let now = <system::Module<T>>::block_number();

            if locks
                .into_iter()
                .all(|l| now < l.until && new_balance < l.amount && l.reasons.contains(reason))
            {
                return Err("account liquidity restrictions prevent withdrawal");
            }
        }

        Ok(())
    }

    // TODO: add fee
    fn transfer(transactor: &T::AccountId, dest: &T::AccountId, value: Self::Balance) -> SResult {
        let new_from_balance = Self::free_balance(transactor)
            .checked_sub(&value)
            .ok_or("balance too low to send value")?;

        Self::ensure_can_withdraw(
            transactor,
            value,
            WithdrawReason::Transfer,
            new_from_balance,
        )?;

        // NOTE: total stake being stored in the same type means that this could never overflow
        // but better to be safe than sorry.
        let new_to_balance = Self::free_balance(dest)
            .checked_add(&value)
            .ok_or("destination balance too high to receive value")?;

        if transactor != dest {
            Self::set_free_balance(transactor, new_from_balance);
            Self::set_free_balance(dest, new_to_balance);
        }

        Self::deposit_event(RawEvent::TokenTransfer(
            transactor.clone(),
            dest.clone(),
            value,
        ));

        Ok(())
    }

    fn slash(who: &T::AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        let free_balance = Self::free_balance(who);
        let free_slash = free_balance.min(value);

        Self::set_free_balance(who, free_balance - free_slash);

        let remaining_slash = value - free_slash;
        if remaining_slash.is_zero() {
            (NegativeImbalance::new(value), 0.into())
        } else {
            let reserved_balance = Self::reserved_balance(who);
            let reserved_slash = reserved_balance.min(remaining_slash);

            Self::set_reserved_balance(who, reserved_balance - reserved_slash);

            (
                NegativeImbalance::new(free_slash + reserved_slash),
                remaining_slash - reserved_slash,
            )
        }
    }

    fn deposit_into_existing(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Result<Self::PositiveImbalance, &'static str> {
        if Self::total_balance(who).is_zero() {
            return Err("beneficiary account must pre-exist");
        }

        // add here
        let balance = Self::free_balance(who);
        let new_balance = balance + value;
        Self::set_free_balance(who, new_balance);

        Ok(PositiveImbalance::new(value))
    }

    fn deposit_creating(who: &T::AccountId, value: Self::Balance) -> Self::PositiveImbalance {
        let (imbalance, _) = Self::make_free_balance_be(who, Self::free_balance(who) + value);

        if let SignedImbalance::Positive(p) = imbalance {
            p
        } else {
            // Impossible, but be defensive.
            Self::PositiveImbalance::zero()
        }
    }

    fn withdraw(
        who: &T::AccountId,
        value: Self::Balance,
        reason: WithdrawReason,
        liveness: ExistenceRequirement,
    ) -> Result<Self::NegativeImbalance, &'static str> {
        if let Some(new_balance) = Self::free_balance(who).checked_sub(&value) {
            if (liveness == ExistenceRequirement::KeepAlive)
                && (new_balance < Self::minimum_balance())
            {
                return Err("payment would kill account");
            }

            Self::ensure_can_withdraw(who, value, reason, new_balance)?;
            Self::set_free_balance(who, new_balance);

            Ok(NegativeImbalance::new(value))
        } else {
            Err("too few free funds in account")
        }
    }

    fn make_free_balance_be(
        who: &T::AccountId,
        balance: Self::Balance,
    ) -> (
        SignedImbalance<Self::Balance, Self::PositiveImbalance>,
        UpdateBalanceOutcome,
    ) {
        let original = Self::free_balance(who);
        let imbalance = if original <= balance {
            SignedImbalance::Positive(PositiveImbalance::new(balance - original))
        } else {
            SignedImbalance::Negative(NegativeImbalance::new(original - balance))
        };

        Self::set_free_balance(who, balance);

        (imbalance, UpdateBalanceOutcome::Updated)
    }
}

impl<T: Trait> LockableCurrency<T::AccountId> for Module<T>
where
    T::Balance: MaybeSerializeDebug,
{
    type Moment = T::BlockNumber;

    fn set_lock(
        id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        until: T::BlockNumber,
        reasons: WithdrawReasons,
    ) {
        let now = <system::Module<T>>::block_number();
        let mut new_lock = Some(BalanceLock {
            id,
            amount,
            until,
            reasons,
        });
        let mut locks: Vec<_> = Self::locks(who)
            .into_iter()
            .filter_map(|lock| {
                if lock.id == id {
                    new_lock.take()
                } else if lock.until > now {
                    Some(lock)
                } else {
                    None
                }
            })
            .collect();

        if let Some(lock) = new_lock {
            locks.push(lock);
        }

        <Locks<T>>::insert(who, locks);
    }

    fn extend_lock(
        id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        until: T::BlockNumber,
        reasons: WithdrawReasons,
    ) {
        let now = <system::Module<T>>::block_number();
        let mut new_lock = Some(BalanceLock {
            id,
            amount,
            until,
            reasons,
        });
        let mut locks: Vec<_> = Self::locks(who)
            .into_iter()
            .filter_map(|lock| {
                if lock.id == id {
                    new_lock.take().map(|old_lock| BalanceLock {
                        id,
                        amount: lock.amount.max(old_lock.amount),
                        until: lock.until.max(old_lock.until),
                        reasons: lock.reasons | old_lock.reasons,
                    })
                } else if lock.until > now {
                    Some(lock)
                } else {
                    None
                }
            })
            .collect();

        if let Some(lock) = new_lock {
            locks.push(lock);
        }

        <Locks<T>>::insert(who, locks);
    }

    fn remove_lock(id: LockIdentifier, who: &T::AccountId) {
        let now = <system::Module<T>>::block_number();
        <Locks<T>>::mutate(who, |locks| {
            locks.retain(|lock| (lock.until > now) && (lock.id != id));
        });
    }
}
