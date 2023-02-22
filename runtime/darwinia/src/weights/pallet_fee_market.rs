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
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("darwinia-local"), DB CACHE: 1024

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
// darwinia-local
// --output
// runtime/darwinia/src/weights/
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
	// Storage: CrabFeeMarket Relayers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	// Storage: CrabFeeMarket Orders (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	fn enroll_and_lock_collateral() -> Weight {
		// Minimum execution time: 98_604 nanoseconds.
		Weight::from_ref_time(98_604_000)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: CrabFeeMarket Relayers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: CrabFeeMarket Orders (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	fn increase_locked_collateral() -> Weight {
		// Minimum execution time: 98_488 nanoseconds.
		Weight::from_ref_time(98_488_000)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: CrabFeeMarket Relayers (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	// Storage: CrabFeeMarket Orders (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	fn decrease_locked_collateral() -> Weight {
		// Minimum execution time: 103_486 nanoseconds.
		Weight::from_ref_time(103_486_000)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: CrabFeeMarket Relayers (r:1 w:0)
	// Storage: CrabFeeMarket RelayersMap (r:10 w:1)
	// Storage: CrabFeeMarket Orders (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	fn update_relay_fee() -> Weight {
		// Minimum execution time: 82_189 nanoseconds.
		Weight::from_ref_time(82_189_000)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CrabFeeMarket Relayers (r:1 w:1)
	// Storage: CrabFeeMarket Orders (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CrabFeeMarket AssignedRelayers (r:1 w:1)
	// Storage: CrabFeeMarket RelayersMap (r:9 w:1)
	// Storage: CrabFeeMarket AssignedRelayersNumber (r:1 w:0)
	fn cancel_enrollment() -> Weight {
		// Minimum execution time: 100_402 nanoseconds.
		Weight::from_ref_time(100_402_000)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: CrabFeeMarket CollateralSlashProtect (r:0 w:1)
	fn set_slash_protect() -> Weight {
		// Minimum execution time: 30_534 nanoseconds.
		Weight::from_ref_time(30_534_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CrabFeeMarket Relayers (r:1 w:0)
	// Storage: CrabFeeMarket RelayersMap (r:10 w:0)
	// Storage: CrabFeeMarket Orders (r:1 w:0)
	// Storage: CrabFeeMarket AssignedRelayers (r:0 w:1)
	// Storage: CrabFeeMarket AssignedRelayersNumber (r:0 w:1)
	fn set_assigned_relayers_number() -> Weight {
		// Minimum execution time: 78_289 nanoseconds.
		Weight::from_ref_time(78_289_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
