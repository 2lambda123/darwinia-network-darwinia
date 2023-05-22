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

//! Autogenerated weights for `darwinia_account_migration`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-29, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("darwinia-local"), DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --execution
// wasm
// --heap-pages
// 4096
// --chain
// darwinia-local
// --output
// runtime/darwinia/src/weights
// --extrinsic
// *
// --pallet
// darwinia-account-migration

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `darwinia_account_migration`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> darwinia_account_migration::WeightInfo for WeightInfo<T> {
	/// Storage: AccountMigration Accounts (r:1 w:1)
	/// Proof: AccountMigration Accounts (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: AccountMigration KtonAccounts (r:1 w:1)
	/// Proof: AccountMigration KtonAccounts (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	/// Storage: AccountMigration Vestings (r:1 w:1)
	/// Proof Skipped: AccountMigration Vestings (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: AccountMigration Identities (r:1 w:1)
	/// Proof: AccountMigration Identities (max_values: None, max_size: Some(9219), added: 11694, mode: MaxEncodedLen)
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// Storage: AccountMigration Ledgers (r:1 w:1)
	/// Proof: AccountMigration Ledgers (max_values: None, max_size: Some(852), added: 3327, mode: MaxEncodedLen)
	/// Storage: AccountMigration Deposits (r:1 w:1)
	/// Proof Skipped: AccountMigration Deposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(94), added: 2569, mode: MaxEncodedLen)
	/// Storage: DarwiniaStaking Ledgers (r:0 w:1)
	/// Proof: DarwiniaStaking Ledgers (max_values: None, max_size: Some(840), added: 3315, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:0 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Vesting Vesting (r:0 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Deposit Deposits (r:0 w:1)
	/// Proof: Deposit Deposits (max_values: None, max_size: Some(853), added: 3328, mode: MaxEncodedLen)
	fn migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5668`
		//  Estimated: `52017`
		// Minimum execution time: 235_808 nanoseconds.
		Weight::from_parts(235_808_000, 52017)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(18))
	}
	/// Storage: AccountMigration Multisigs (r:0 w:1)
	/// Proof Skipped: AccountMigration Multisigs (max_values: None, max_size: None, mode: Measured)
	/// Storage: AccountMigration Accounts (r:1 w:1)
	/// Proof: AccountMigration Accounts (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: AccountMigration KtonAccounts (r:1 w:1)
	/// Proof: AccountMigration KtonAccounts (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(166), added: 2641, mode: MaxEncodedLen)
	/// Storage: AccountMigration Vestings (r:1 w:1)
	/// Proof Skipped: AccountMigration Vestings (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: AccountMigration Identities (r:1 w:1)
	/// Proof: AccountMigration Identities (max_values: None, max_size: Some(9219), added: 11694, mode: MaxEncodedLen)
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// Storage: AccountMigration Ledgers (r:1 w:1)
	/// Proof: AccountMigration Ledgers (max_values: None, max_size: Some(852), added: 3327, mode: MaxEncodedLen)
	/// Storage: AccountMigration Deposits (r:1 w:1)
	/// Proof Skipped: AccountMigration Deposits (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:2)
	/// Proof: Assets Account (max_values: None, max_size: Some(94), added: 2569, mode: MaxEncodedLen)
	/// Storage: DarwiniaStaking Ledgers (r:0 w:1)
	/// Proof: DarwiniaStaking Ledgers (max_values: None, max_size: Some(840), added: 3315, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:0 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Vesting Vesting (r:0 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Deposit Deposits (r:0 w:1)
	/// Proof: Deposit Deposits (max_values: None, max_size: Some(853), added: 3328, mode: MaxEncodedLen)
	/// The range of component `x` is `[0, 99]`.
	/// The range of component `y` is `[0, 99]`.
	/// The range of component `z` is `[0, 99]`.
	fn migrate_multisig(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5329 + z * (3 ±0)`
		//  Estimated: `52017 + y * (7 ±0) + z * (3 ±0)`
		// Minimum execution time: 33_160 nanoseconds.
		Weight::from_parts(197_482_666, 52017)
			// Standard Error: 134_286
			.saturating_add(Weight::from_parts(176_269, 0).saturating_mul(x.into()))
			// Standard Error: 134_286
			.saturating_add(Weight::from_parts(15_875, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(18))
			.saturating_add(Weight::from_parts(7, 0).saturating_mul(y.into()))
			.saturating_add(Weight::from_parts(3, 0).saturating_mul(z.into()))
	}
	/// Storage: AccountMigration Multisigs (r:1 w:1)
	/// Proof Skipped: AccountMigration Multisigs (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn complete_multisig_migration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4881`
		//  Estimated: `9947`
		// Minimum execution time: 21_179 nanoseconds.
		Weight::from_parts(21_179_000, 9947)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
