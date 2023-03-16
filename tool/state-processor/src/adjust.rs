// darwinia
use crate::*;

pub trait Adjust {
	fn adjust(&mut self);
}

impl Adjust for BlockNumber {
	fn adjust(&mut self) {
		// https://github.com/darwinia-network/darwinia-2.0/issues/78
		*self = self.checked_sub(*NOW.read().unwrap()).unwrap_or_default() / 2;
	}
}

impl Adjust for Balance {
	fn adjust(&mut self) {
		*self *= GWEI;
	}
}

impl Adjust for AccountData {
	fn adjust(&mut self) {
		self.free.adjust();
		self.reserved.adjust();
		self.free_kton_or_misc_frozen.adjust();
		self.reserved_kton_or_fee_frozen.adjust();
	}
}

impl Adjust for BalanceLock {
	fn adjust(&mut self) {
		self.amount.adjust();
	}
}

impl Adjust for VestingInfo {
	fn adjust(&mut self) {
		let now = *NOW.read().unwrap();
		let released = (now - self.starting_block) as Balance * self.per_block;

		if self.locked <= released {
			self.locked = 0;

			return;
		} else {
			self.locked -= released;
		}

		self.locked.adjust();
		self.per_block.adjust();
		self.per_block *= 2;
		self.starting_block = 0;
	}
}

impl Adjust for StakingLedger {
	fn adjust(&mut self) {
		self.active.adjust();
		self.active_deposit_ring.adjust();
		self.active_kton.adjust();
		self.deposit_items.iter_mut().for_each(Adjust::adjust);
		self.ring_staking_lock.adjust();
		self.kton_staking_lock.adjust();
	}
}
impl Adjust for TimeDepositItem {
	fn adjust(&mut self) {
		self.value.adjust();
	}
}
impl Adjust for StakingLock {
	fn adjust(&mut self) {
		self.unbondings.iter_mut().for_each(Adjust::adjust);
	}
}
impl Adjust for Unbonding {
	fn adjust(&mut self) {
		self.amount.adjust();
		self.until.adjust();
	}
}

impl Adjust for Registration {
	fn adjust(&mut self) {
		self.judgements.iter_mut().for_each(|(_, judgement)| {
			if let Judgement::FeePaid(ref mut amount) = judgement {
				amount.adjust();
			}
		});
		self.deposit.adjust();
	}
}

impl Adjust for RegistrarInfo<AccountId32> {
	fn adjust(&mut self) {
		self.fee.adjust();
	}
}
