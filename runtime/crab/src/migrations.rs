// --- paritytech ---
#[allow(unused)]
use frame_support::{migration, traits::OnRuntimeUpgrade, weights::Weight};
// --- darwinia-network ---
#[allow(unused)]
use crate::*;

pub struct CustomOnRuntimeUpgrade;
impl OnRuntimeUpgrade for CustomOnRuntimeUpgrade {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		Ok(())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		Ok(())
	}

	fn on_runtime_upgrade() -> Weight {
		migrate()
	}
}

fn migrate() -> Weight {
	// --- darwinia-network ---
	use darwinia_staking::{StakingLedger, STAKING_ID};

	let now = System::block_number();

	migration::storage_iter::<StakingLedger<AccountId, Balance, Balance, BlockNumber>>(
		b"Staking", b"Ledger",
	)
	.for_each(
		|(
			_k,
			StakingLedger {
				stash, active, active_kton, ring_staking_lock, kton_staking_lock, ..
			},
		)| {
			let all_ring_lock = ring_staking_lock.total_unbond();
			let valid_ring_lock = ring_staking_lock.total_unbond_at(now);

			if all_ring_lock != valid_ring_lock {
				<darwinia_balances::Locks<Runtime, RingInstance>>::mutate(&stash, |locks| {
					// `WeakBoundedVec` only implement `IndexMut`, otherwise we can use `iter_mut`
					// here.
					for i in 0..locks.len() {
						let lock = &mut locks[i];

						if lock.id == STAKING_ID {
							lock.amount = active + valid_ring_lock;
						}
					}
				});
			}

			let all_kton_lock = kton_staking_lock.total_unbond();
			let valid_kton_lock = kton_staking_lock.total_unbond_at(now);

			if all_kton_lock != valid_kton_lock {
				<darwinia_balances::Locks<Runtime, KtonInstance>>::mutate(&stash, |locks| {
					// `WeakBoundedVec` only implement `IndexMut`, otherwise we can use `iter_mut`
					// here.
					for i in 0..locks.len() {
						let lock = &mut locks[i];

						if lock.id == STAKING_ID {
							lock.amount = active_kton + valid_kton_lock;
						}
					}
				});
			}
		},
	);

	// 0
	RuntimeBlockWeights::get().max_block
}
