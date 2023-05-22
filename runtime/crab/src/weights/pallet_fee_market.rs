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
//! DATE: 2023-02-22, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Debian`, CPU: `12th Gen Intel(R) Core(TM) i9-12900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("crab-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --execution
// wasm
// --heap-pages
// 4096
// --steps
// 2
// --repeat
// 1
// --chain
// crab-local
// --output
// runtime/crab/src/weights/
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
	// Storage: DarwiniaFeeMarket Relayers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DarwiniaFeeMarket RelayersMap (r:10 w:1)
	// Storage: DarwiniaFeeMarket Orders (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayers (r:0 w:1)
	fn enroll_and_lock_collateral() -> Weight {
		// Minimum execution time: 101_642 nanoseconds.
		Weight::from_parts(101_642_000, 0)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: DarwiniaFeeMarket Relayers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: DarwiniaFeeMarket RelayersMap (r:10 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DarwiniaFeeMarket Orders (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayers (r:0 w:1)
	fn increase_locked_collateral() -> Weight {
		// Minimum execution time: 99_179 nanoseconds.
		Weight::from_parts(99_179_000, 0)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: DarwiniaFeeMarket Relayers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: DarwiniaFeeMarket RelayersMap (r:10 w:1)
	// Storage: DarwiniaFeeMarket Orders (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DarwiniaFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayers (r:0 w:1)
	fn decrease_locked_collateral() -> Weight {
		// Minimum execution time: 111_459 nanoseconds.
		Weight::from_parts(111_459_000, 0)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: DarwiniaFeeMarket Relayers (r:1 w:0)
	// Storage: DarwiniaFeeMarket RelayersMap (r:10 w:1)
	// Storage: DarwiniaFeeMarket Orders (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayers (r:0 w:1)
	fn update_relay_fee() -> Weight {
		// Minimum execution time: 95_315 nanoseconds.
		Weight::from_parts(95_315_000, 0)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: DarwiniaFeeMarket Relayers (r:1 w:1)
	// Storage: DarwiniaFeeMarket Orders (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DarwiniaFeeMarket AssignedRelayers (r:1 w:1)
	// Storage: DarwiniaFeeMarket RelayersMap (r:9 w:1)
	// Storage: DarwiniaFeeMarket AssignedRelayersNumber (r:1 w:0)
	fn cancel_enrollment() -> Weight {
		// Minimum execution time: 163_042 nanoseconds.
		Weight::from_parts(163_042_000, 0)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: DarwiniaFeeMarket CollateralSlashProtect (r:0 w:1)
	fn set_slash_protect() -> Weight {
		// Minimum execution time: 31_462 nanoseconds.
		Weight::from_parts(31_462_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: DarwiniaFeeMarket Relayers (r:1 w:0)
	// Storage: DarwiniaFeeMarket RelayersMap (r:10 w:0)
	// Storage: DarwiniaFeeMarket Orders (r:1 w:0)
	// Storage: DarwiniaFeeMarket AssignedRelayers (r:0 w:1)
	// Storage: DarwiniaFeeMarket AssignedRelayersNumber (r:0 w:1)
	fn set_assigned_relayers_number() -> Weight {
		// Minimum execution time: 77_980 nanoseconds.
		Weight::from_parts(77_980_000, 0)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
