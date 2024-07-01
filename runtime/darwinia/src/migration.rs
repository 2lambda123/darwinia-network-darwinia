// This file is part of Darwinia.
//
// Copyright (C) Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

// darwinia
#[allow(unused_imports)]
use crate::*;
// polkadot-sdk
#[allow(unused_imports)]
use frame_support::{migration, storage::unhashed};

pub struct CustomOnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for CustomOnRuntimeUpgrade {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
		log::info!("pre");

		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		log::info!("post");

		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		migrate()
	}
}

fn migrate() -> frame_support::weights::Weight {
	// core
	use core::str::FromStr;

	const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];
	// DOT equals to the 0x405 in the pallet-evm runtime.
	const ADDRESS: &str = "0x0000000000000000000000000000000000000405";

	if let Ok(addr) = H160::from_str(ADDRESS) {
		EVM::create_account(addr, REVERT_BYTECODE.to_vec());
	}

	let _ = migration::clear_storage_prefix(
		b"BridgeKusamaGrandpa",
		b"ImportedHeaders",
		&[],
		Some(100),
		None,
	);
	let mut w = 103;

	w += migration_helper::PalletCleaner {
		name: b"Council",
		values: &[b"Proposals", b"ProposalCount", b"Members", b"Prime"],
		maps: &[b"ProposalOf", b"Voting"],
	}
	.remove_all();
	w += migration_helper::PalletCleaner {
		name: b"Democracy",
		values: &[
			b"PublicPropCount",
			b"PublicProps",
			b"ReferendumCount",
			b"LowestUnbaked",
			b"LastTabledWasExternal",
			b"NextExternal",
		],
		maps: &[
			b"DepositOf",
			b"ReferendumInfoOf",
			b"VotingOf",
			b"Blacklist",
			b"Cancellations",
			b"MetadataOf",
		],
	}
	.remove_all();
	w += migration_helper::PalletCleaner {
		name: b"Identity",
		values: &[b"Registrars"],
		maps: &[b"SuperOf", b"SubsOf", b"Registrars"],
	}
	.remove_all();
	w += migration_helper::migrate_identity_of::<pallet_balances::Pallet<Runtime>>();

	// frame_support::weights::Weight::zero()
	<Runtime as frame_system::Config>::DbWeight::get().reads_writes(2, w)
}
