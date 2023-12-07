// This file is part of Darwinia.
//
// Copyright (C) 2018-2023 Darwinia Network
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

//! Autogenerated weights for darwinia_account_migration
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `Intel(R) Core(TM) i9-14900KF`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("pangolin-dev"), DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --template
// .maintain/pallet-weight-template.hbs
// --heap-pages
// 4096
// --chain
// pangolin-dev
// --output
// pallet/account-migration/src/weights.rs
// --extrinsic
// *
// --pallet
// darwinia-account-migration

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for darwinia_account_migration.
pub trait WeightInfo {
	fn migrate() -> Weight;
	fn migrate_multisig(x: u32, y: u32, z: u32, ) -> Weight;
	fn complete_multisig_migration() -> Weight;
}

/// Weights for darwinia_account_migration using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `AccountMigration::Accounts` (r:1 w:1)
	/// Proof: `AccountMigration::Accounts` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::KtonAccounts` (r:1 w:1)
	/// Proof: `AccountMigration::KtonAccounts` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Identities` (r:1 w:1)
	/// Proof: `AccountMigration::Identities` (`max_values`: None, `max_size`: Some(9219), added: 11694, mode: `MaxEncodedLen`)
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Ledgers` (r:1 w:1)
	/// Proof: `AccountMigration::Ledgers` (`max_values`: None, `max_size`: Some(1845), added: 4320, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Deposits` (r:1 w:1)
	/// Proof: `AccountMigration::Deposits` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Ledgers` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:0 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:0 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	fn migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `29212`
		//  Estimated: `32677`
		// Minimum execution time: 143_334 nanoseconds.
		Weight::from_parts(149_012_000, 0)
			.saturating_add(Weight::from_parts(32677, 0))
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(15_u64))
	}
	/// Storage: `AccountMigration::Multisigs` (r:0 w:1)
	/// Proof: `AccountMigration::Multisigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AccountMigration::Accounts` (r:1 w:1)
	/// Proof: `AccountMigration::Accounts` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::KtonAccounts` (r:1 w:1)
	/// Proof: `AccountMigration::KtonAccounts` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Identities` (r:1 w:1)
	/// Proof: `AccountMigration::Identities` (`max_values`: None, `max_size`: Some(9219), added: 11694, mode: `MaxEncodedLen`)
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Ledgers` (r:1 w:1)
	/// Proof: `AccountMigration::Ledgers` (`max_values`: None, `max_size`: Some(1845), added: 4320, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Deposits` (r:1 w:1)
	/// Proof: `AccountMigration::Deposits` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Ledgers` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:0 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:0 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 99]`.
	/// The range of component `y` is `[0, 99]`.
	/// The range of component `z` is `[0, 99]`.
	fn migrate_multisig(x: u32, _y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `2 + x * (165 ±0) + z * (165 ±0)`
		// Minimum execution time: 6_532 nanoseconds.
		Weight::from_parts(21_062_269, 0)
			.saturating_add(Weight::from_parts(2, 0))
			// Standard Error: 11_776
			.saturating_add(Weight::from_parts(4_768, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(165, 0).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(165, 0).saturating_mul(z.into()))
	}
	/// Storage: `AccountMigration::Multisigs` (r:1 w:1)
	/// Proof: `AccountMigration::Multisigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn complete_multisig_migration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3499`
		//  Estimated: `6964`
		// Minimum execution time: 7_617 nanoseconds.
		Weight::from_parts(8_366_000, 0)
			.saturating_add(Weight::from_parts(6964, 0))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `AccountMigration::Accounts` (r:1 w:1)
	/// Proof: `AccountMigration::Accounts` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::KtonAccounts` (r:1 w:1)
	/// Proof: `AccountMigration::KtonAccounts` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Identities` (r:1 w:1)
	/// Proof: `AccountMigration::Identities` (`max_values`: None, `max_size`: Some(9219), added: 11694, mode: `MaxEncodedLen`)
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Ledgers` (r:1 w:1)
	/// Proof: `AccountMigration::Ledgers` (`max_values`: None, `max_size`: Some(1845), added: 4320, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Deposits` (r:1 w:1)
	/// Proof: `AccountMigration::Deposits` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Ledgers` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:0 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:0 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	fn migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `29212`
		//  Estimated: `32677`
		// Minimum execution time: 143_334 nanoseconds.
		Weight::from_parts(149_012_000, 0)
			.saturating_add(Weight::from_parts(32677, 0))
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(15_u64))
	}
	/// Storage: `AccountMigration::Multisigs` (r:0 w:1)
	/// Proof: `AccountMigration::Multisigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AccountMigration::Accounts` (r:1 w:1)
	/// Proof: `AccountMigration::Accounts` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::KtonAccounts` (r:1 w:1)
	/// Proof: `AccountMigration::KtonAccounts` (`max_values`: None, `max_size`: Some(82), added: 2557, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Identities` (r:1 w:1)
	/// Proof: `AccountMigration::Identities` (`max_values`: None, `max_size`: Some(9219), added: 11694, mode: `MaxEncodedLen`)
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(901), added: 1396, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Ledgers` (r:1 w:1)
	/// Proof: `AccountMigration::Ledgers` (`max_values`: None, `max_size`: Some(1845), added: 4320, mode: `MaxEncodedLen`)
	/// Storage: `AccountMigration::Deposits` (r:1 w:1)
	/// Proof: `AccountMigration::Deposits` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Ledgers` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1833), added: 4308, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:0 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7526), added: 10001, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:0 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 99]`.
	/// The range of component `y` is `[0, 99]`.
	/// The range of component `z` is `[0, 99]`.
	fn migrate_multisig(x: u32, _y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `2 + x * (165 ±0) + z * (165 ±0)`
		// Minimum execution time: 6_532 nanoseconds.
		Weight::from_parts(21_062_269, 0)
			.saturating_add(Weight::from_parts(2, 0))
			// Standard Error: 11_776
			.saturating_add(Weight::from_parts(4_768, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(165, 0).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(165, 0).saturating_mul(z.into()))
	}
	/// Storage: `AccountMigration::Multisigs` (r:1 w:1)
	/// Proof: `AccountMigration::Multisigs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn complete_multisig_migration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3499`
		//  Estimated: `6964`
		// Minimum execution time: 7_617 nanoseconds.
		Weight::from_parts(8_366_000, 0)
			.saturating_add(Weight::from_parts(6964, 0))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
