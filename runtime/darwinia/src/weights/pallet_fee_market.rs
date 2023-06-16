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

//! Autogenerated weights for `pallet_fee_market`
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

/// Weight functions for `pallet_fee_market`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fee_market::WeightInfo for WeightInfo<T> {
	/// Storage: CrabFeeMarket Relayers (r:1 w:1)
	/// Proof Skipped: CrabFeeMarket Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	/// Proof Skipped: CrabFeeMarket RelayersMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket Orders (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Orders (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket AssignedRelayersNumber (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayers (max_values: Some(1), max_size: None, mode: Measured)
	fn enroll_and_lock_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1649`
		//  Estimated: `48753`
		// Minimum execution time: 92_950_000 picoseconds.
		Weight::from_parts(92_950_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: CrabFeeMarket Relayers (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	/// Proof Skipped: CrabFeeMarket RelayersMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: CrabFeeMarket Orders (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Orders (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket AssignedRelayersNumber (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayers (max_values: Some(1), max_size: None, mode: Measured)
	fn increase_locked_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1721`
		//  Estimated: `49113`
		// Minimum execution time: 88_969_000 picoseconds.
		Weight::from_parts(88_969_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: CrabFeeMarket Relayers (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	/// Proof Skipped: CrabFeeMarket RelayersMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket Orders (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Orders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket AssignedRelayersNumber (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayers (max_values: Some(1), max_size: None, mode: Measured)
	fn decrease_locked_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1721`
		//  Estimated: `49113`
		// Minimum execution time: 102_776_000 picoseconds.
		Weight::from_parts(102_776_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: CrabFeeMarket Relayers (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	/// Proof Skipped: CrabFeeMarket RelayersMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket Orders (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Orders (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket AssignedRelayersNumber (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayers (max_values: Some(1), max_size: None, mode: Measured)
	fn update_relay_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1302`
		//  Estimated: `38685`
		// Minimum execution time: 74_835_000 picoseconds.
		Weight::from_parts(74_835_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: CrabFeeMarket Relayers (r:1 w:1)
	/// Proof Skipped: CrabFeeMarket Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket Orders (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Orders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: CrabFeeMarket AssignedRelayers (r:1 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket RelayersMap (r:9 w:1)
	/// Proof Skipped: CrabFeeMarket RelayersMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket AssignedRelayersNumber (max_values: Some(1), max_size: None, mode: Measured)
	fn cancel_enrollment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1788`
		//  Estimated: `48458`
		// Minimum execution time: 148_277_000 picoseconds.
		Weight::from_parts(148_277_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: CrabFeeMarket CollateralSlashProtect (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket CollateralSlashProtect (max_values: Some(1), max_size: None, mode: Measured)
	fn set_slash_protect() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 220_176_000 picoseconds.
		Weight::from_parts(220_176_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CrabFeeMarket Relayers (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket RelayersMap (r:10 w:0)
	/// Proof Skipped: CrabFeeMarket RelayersMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket Orders (r:1 w:0)
	/// Proof Skipped: CrabFeeMarket Orders (max_values: None, max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CrabFeeMarket AssignedRelayersNumber (r:0 w:1)
	/// Proof Skipped: CrabFeeMarket AssignedRelayersNumber (max_values: Some(1), max_size: None, mode: Measured)
	fn set_assigned_relayers_number() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1302`
		//  Estimated: `37200`
		// Minimum execution time: 71_350_000 picoseconds.
		Weight::from_parts(71_350_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
