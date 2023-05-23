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

//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-23, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("pangolin-local"), DB CACHE: 1024

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
// pangolin-local
// --output
// runtime/pangolin/src/weights
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(_l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9262`
		// Minimum execution time: 38_503_000 picoseconds.
		Weight::from_parts(40_704_555, 0)
			.saturating_add(Weight::from_parts(0, 9262))
			// Standard Error: 145_780
			.saturating_add(Weight::from_parts(164_944, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9262`
		// Minimum execution time: 39_429_000 picoseconds.
		Weight::from_parts(34_989_444, 0)
			.saturating_add(Weight::from_parts(0, 9262))
			// Standard Error: 101_731
			.saturating_add(Weight::from_parts(91_551, 0).saturating_mul(l.into()))
			// Standard Error: 184_623
			.saturating_add(Weight::from_parts(158_555, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `404 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `12843`
		// Minimum execution time: 40_653_000 picoseconds.
		Weight::from_parts(35_571_888, 0)
			.saturating_add(Weight::from_parts(0, 12843))
			// Standard Error: 22_781
			.saturating_add(Weight::from_parts(97_030, 0).saturating_mul(l.into()))
			// Standard Error: 41_344
			.saturating_add(Weight::from_parts(326_611, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `404 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `12843`
		// Minimum execution time: 39_732_000 picoseconds.
		Weight::from_parts(32_951_333, 0)
			.saturating_add(Weight::from_parts(0, 12843))
			// Standard Error: 53_994
			.saturating_add(Weight::from_parts(147_867, 0).saturating_mul(l.into()))
			// Standard Error: 97_989
			.saturating_add(Weight::from_parts(242_166, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `198 + l * (25 ±0) + s * (42 ±0)`
		//  Estimated: `12843`
		// Minimum execution time: 57_245_000 picoseconds.
		Weight::from_parts(71_394_000, 0)
			.saturating_add(Weight::from_parts(0, 12843))
			// Standard Error: 111_628
			.saturating_add(Weight::from_parts(16_204, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360 + l * (25 ±0) + s * (42 ±0)`
		//  Estimated: `15434`
		// Minimum execution time: 60_257_000 picoseconds.
		Weight::from_parts(97_115_500, 0)
			.saturating_add(Weight::from_parts(0, 15434))
			// Standard Error: 12_071
			.saturating_add(Weight::from_parts(93_785, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(_l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `410 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `12843`
		// Minimum execution time: 43_297_000 picoseconds.
		Weight::from_parts(51_116_769, 0)
			.saturating_add(Weight::from_parts(0, 12843))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1045), added: 3520, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `410 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `12843`
		// Minimum execution time: 42_573_000 picoseconds.
		Weight::from_parts(45_357_384, 0)
			.saturating_add(Weight::from_parts(0, 12843))
			// Standard Error: 31_371
			.saturating_add(Weight::from_parts(29_255, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
