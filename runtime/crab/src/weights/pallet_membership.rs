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

//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-23, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("crab-local"), DB CACHE: 1024

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
// crab-local
// --output
// runtime/crab/src/weights
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(2002), added: 2497, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162 + m * (40 ±0)`
		//  Estimated: `17365`
		// Minimum execution time: 25_698_000 picoseconds.
		Weight::from_parts(25_821_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(2002), added: 2497, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246 + m * (40 ±0)`
		//  Estimated: `19230`
		// Minimum execution time: 27_980_000 picoseconds.
		Weight::from_parts(28_777_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(2002), added: 2497, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246 + m * (40 ±0)`
		//  Estimated: `19230`
		// Minimum execution time: 29_762_000 picoseconds.
		Weight::from_parts(32_796_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(2002), added: 2497, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:0)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242 + m * (40 ±0)`
		//  Estimated: `19230`
		// Minimum execution time: 28_343_000 picoseconds.
		Weight::from_parts(39_363_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: TechnicalMembership Members (r:1 w:1)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(2002), added: 2497, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalMembership Prime (r:1 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Members (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242 + m * (40 ±0)`
		//  Estimated: `19230`
		// Minimum execution time: 31_438_000 picoseconds.
		Weight::from_parts(36_896_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TechnicalMembership Members (r:1 w:0)
	/// Proof: TechnicalMembership Members (max_values: Some(1), max_size: Some(2002), added: 2497, mode: MaxEncodedLen)
	/// Storage: TechnicalMembership Prime (r:0 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28 + m * (20 ±0)`
		//  Estimated: `5520`
		// Minimum execution time: 14_358_000 picoseconds.
		Weight::from_parts(15_987_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TechnicalMembership Prime (r:0 w:1)
	/// Proof: TechnicalMembership Prime (max_values: Some(1), max_size: Some(20), added: 515, mode: MaxEncodedLen)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(_m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_300_000 picoseconds.
		Weight::from_parts(11_615_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
