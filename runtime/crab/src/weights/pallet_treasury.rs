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

//! Autogenerated weights for `pallet_treasury`
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

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	fn spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 760_000 picoseconds.
		Weight::from_parts(760_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Treasury ProposalCount (r:1 w:1)
	/// Proof: Treasury ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Treasury Proposals (r:0 w:1)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn propose_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `1489`
		// Minimum execution time: 35_337_000 picoseconds.
		Weight::from_parts(35_337_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Treasury Proposals (r:1 w:1)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `504`
		//  Estimated: `9721`
		// Minimum execution time: 40_532_000 picoseconds.
		Weight::from_parts(40_532_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Treasury Proposals (r:1 w:0)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Treasury Approvals (r:1 w:1)
	/// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 99]`.
	fn approve_proposal(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167 + p * (11 ±0)`
		//  Estimated: `5436`
		// Minimum execution time: 14_744_000 picoseconds.
		Weight::from_parts(15_999_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Treasury Approvals (r:1 w:1)
	/// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	fn remove_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `1887`
		// Minimum execution time: 8_178_000 picoseconds.
		Weight::from_parts(8_178_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Treasury Deactivated (r:1 w:1)
	/// Proof: Treasury Deactivated (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:1)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Treasury Approvals (r:1 w:1)
	/// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	/// Storage: Treasury Proposals (r:100 w:0)
	/// Proof: Treasury Proposals (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn on_initialize_proposals(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204 + p * (97 ±0)`
		//  Estimated: `265360`
		// Minimum execution time: 32_675_000 picoseconds.
		Weight::from_parts(236_683_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(104))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
