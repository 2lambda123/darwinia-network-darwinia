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

//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-23, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	/// Storage: Scheduler IncompleteSince (r:1 w:1)
	/// Proof: Scheduler IncompleteSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_agendas_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31`
		//  Estimated: `1489`
		// Minimum execution time: 18_477_000 picoseconds.
		Weight::from_parts(18_477_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4 + s * (178 ±0)`
		//  Estimated: `42428`
		// Minimum execution time: 12_699_000 picoseconds.
		Weight::from_parts(43_089_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_506_000 picoseconds.
		Weight::from_parts(16_506_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172 + s * (1 ±0)`
		//  Estimated: `4201489`
		// Minimum execution time: 32_836_000 picoseconds.
		Weight::from_parts(2_719_750_000, 0)
			.saturating_add(Weight::from_parts(0, 4201489))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn service_task_named() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 19_502_000 picoseconds.
		Weight::from_parts(19_502_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_472_000 picoseconds.
		Weight::from_parts(15_472_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute_dispatch_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 17_450_000 picoseconds.
		Weight::from_parts(17_450_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_363_000 picoseconds.
		Weight::from_parts(15_363_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4 + s * (178 ±0)`
		//  Estimated: `42428`
		// Minimum execution time: 34_193_000 picoseconds.
		Weight::from_parts(56_852_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:0 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78 + s * (177 ±0)`
		//  Estimated: `42428`
		// Minimum execution time: 33_068_000 picoseconds.
		Weight::from_parts(82_038_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4 + s * (191 ±0)`
		//  Estimated: `45941`
		// Minimum execution time: 35_895_000 picoseconds.
		Weight::from_parts(69_820_000, 0)
			.saturating_add(Weight::from_parts(0, 45941))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `102 + s * (188 ±0)`
		//  Estimated: `45941`
		// Minimum execution time: 29_999_000 picoseconds.
		Weight::from_parts(77_889_000, 0)
			.saturating_add(Weight::from_parts(0, 45941))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
