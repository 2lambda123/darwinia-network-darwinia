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

//! Autogenerated weights for `pallet_tx_pause`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-10-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("crab-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --heap-pages
// 4096
// --chain
// crab-dev
// --output
// runtime/crab/src/weights
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

/// Weight functions for `pallet_tx_pause`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tx_pause::WeightInfo for WeightInfo<T> {
	/// Storage: `TxPause::PausedCalls` (r:1 w:1)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	fn pause() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3997`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:1)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	fn unpause() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `566`
		//  Estimated: `3997`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
