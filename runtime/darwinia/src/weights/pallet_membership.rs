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
//! DATE: 2023-11-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `Intel(R) Core(TM) i9-14900KF`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("darwinia-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --heap-pages
// 4096
// --chain
// darwinia-dev
// --output
// runtime/darwinia/src/weights
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: `TechnicalMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalMembership::Members` (`max_values`: Some(1), `max_size`: Some(2002), added: 2497, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135 + m * (40 ±0)`
		//  Estimated: `3487 + m * (40 ±0)`
		// Minimum execution time: 9_983_000 picoseconds.
		Weight::from_parts(10_895_944, 0)
			.saturating_add(Weight::from_parts(0, 3487))
			// Standard Error: 428
			.saturating_add(Weight::from_parts(9_492, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalMembership::Members` (`max_values`: Some(1), `max_size`: Some(2002), added: 2497, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalMembership::Prime` (r:1 w:0)
	/// Proof: `TechnicalMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215 + m * (40 ±0)`
		//  Estimated: `3487 + m * (40 ±0)`
		// Minimum execution time: 12_019_000 picoseconds.
		Weight::from_parts(12_714_497, 0)
			.saturating_add(Weight::from_parts(0, 3487))
			// Standard Error: 502
			.saturating_add(Weight::from_parts(7_082, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalMembership::Members` (`max_values`: Some(1), `max_size`: Some(2002), added: 2497, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalMembership::Prime` (r:1 w:0)
	/// Proof: `TechnicalMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215 + m * (40 ±0)`
		//  Estimated: `3487 + m * (40 ±0)`
		// Minimum execution time: 11_850_000 picoseconds.
		Weight::from_parts(12_671_866, 0)
			.saturating_add(Weight::from_parts(0, 3487))
			// Standard Error: 535
			.saturating_add(Weight::from_parts(15_353, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalMembership::Members` (`max_values`: Some(1), `max_size`: Some(2002), added: 2497, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalMembership::Prime` (r:1 w:0)
	/// Proof: `TechnicalMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215 + m * (40 ±0)`
		//  Estimated: `3487 + m * (40 ±0)`
		// Minimum execution time: 11_421_000 picoseconds.
		Weight::from_parts(13_225_872, 0)
			.saturating_add(Weight::from_parts(0, 3487))
			// Standard Error: 1_130
			.saturating_add(Weight::from_parts(71_185, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalMembership::Members` (`max_values`: Some(1), `max_size`: Some(2002), added: 2497, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalMembership::Prime` (r:1 w:1)
	/// Proof: `TechnicalMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `215 + m * (40 ±0)`
		//  Estimated: `3487 + m * (40 ±0)`
		// Minimum execution time: 12_547_000 picoseconds.
		Weight::from_parts(13_374_340, 0)
			.saturating_add(Weight::from_parts(0, 3487))
			// Standard Error: 504
			.saturating_add(Weight::from_parts(13_432, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalMembership::Members` (r:1 w:0)
	/// Proof: `TechnicalMembership::Members` (`max_values`: Some(1), `max_size`: Some(2002), added: 2497, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalMembership::Prime` (r:0 w:1)
	/// Proof: `TechnicalMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31 + m * (20 ±0)`
		//  Estimated: `3487 + m * (20 ±0)`
		// Minimum execution time: 4_870_000 picoseconds.
		Weight::from_parts(5_346_178, 0)
			.saturating_add(Weight::from_parts(0, 3487))
			// Standard Error: 252
			.saturating_add(Weight::from_parts(4_932, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 20).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalMembership::Prime` (r:0 w:1)
	/// Proof: `TechnicalMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_947_000 picoseconds.
		Weight::from_parts(2_189_548, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 133
			.saturating_add(Weight::from_parts(150, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
