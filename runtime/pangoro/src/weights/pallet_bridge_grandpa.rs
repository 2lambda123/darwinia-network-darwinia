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

//! Autogenerated weights for `pallet_bridge_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Xaviers-MacBook-Pro-16.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("pangoro-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --heap-pages
// 4096
// --chain
// pangoro-dev
// --output
// runtime/pangoro/src/weights
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

/// Weight functions for `pallet_bridge_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_grandpa::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeRococoGrandpa::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoGrandpa::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoGrandpa::RequestCount` (r:1 w:1)
	/// Proof: `BridgeRococoGrandpa::RequestCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoGrandpa::BestFinalized` (r:1 w:1)
	/// Proof: `BridgeRococoGrandpa::BestFinalized` (`max_values`: Some(1), `max_size`: Some(36), added: 531, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoGrandpa::ImportedHeaders` (r:1 w:2)
	/// Proof: `BridgeRococoGrandpa::ImportedHeaders` (`max_values`: None, `max_size`: Some(65568), added: 68043, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoGrandpa::CurrentAuthoritySet` (r:1 w:0)
	/// Proof: `BridgeRococoGrandpa::CurrentAuthoritySet` (`max_values`: Some(1), `max_size`: Some(163850), added: 164345, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoGrandpa::ImportedHashesPointer` (r:1 w:1)
	/// Proof: `BridgeRococoGrandpa::ImportedHashesPointer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoGrandpa::ImportedHashes` (r:1 w:1)
	/// Proof: `BridgeRococoGrandpa::ImportedHashes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[51, 102]`.
	/// The range of component `v` is `[50, 100]`.
	fn submit_finality_proof(p: u32, v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2510 + p * (40 ±0)`
		//  Estimated: `165335`
		// Minimum execution time: 1_749_000_000 picoseconds.
		Weight::from_parts(104_176_377, 0)
			.saturating_add(Weight::from_parts(0, 165335))
			// Standard Error: 55_218
			.saturating_add(Weight::from_parts(29_928_835, 0).saturating_mul(p.into()))
			// Standard Error: 56_528
			.saturating_add(Weight::from_parts(1_862_181, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
