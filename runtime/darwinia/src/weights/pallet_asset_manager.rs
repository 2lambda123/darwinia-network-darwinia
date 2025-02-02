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

//! Autogenerated weights for `pallet_asset_manager`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
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

/// Weight functions for `pallet_asset_manager`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_manager::WeightInfo for WeightInfo<T> {
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(166), added: 2641, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(144), added: 2619, mode: `MaxEncodedLen`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn register_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `3741`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3741))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AssetManager::AssetTypeId` (r:1 w:0)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn set_asset_units_per_second(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `595 + x * (9 ±0)`
		//  Estimated: `3983 + x * (10 ±0)`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(17_795_668, 0)
			.saturating_add(Weight::from_parts(0, 3983))
			// Standard Error: 5_207
			.saturating_add(Weight::from_parts(392_816, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:1 w:2)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:2)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn change_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `968 + x * (13 ±0)`
		//  Estimated: `4303 + x * (14 ±0)`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(28_440_466, 0)
			.saturating_add(Weight::from_parts(0, 4303))
			// Standard Error: 6_795
			.saturating_add(Weight::from_parts(386_150, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(Weight::from_parts(0, 14).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn remove_supported_asset(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + x * (5 ±0)`
		//  Estimated: `1678 + x * (5 ±0)`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(11_889_301, 0)
			.saturating_add(Weight::from_parts(0, 1678))
			// Standard Error: 4_468
			.saturating_add(Weight::from_parts(336_614, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(x.into()))
	}
	/// Storage: `AssetManager::SupportedFeePaymentAssets` (r:1 w:1)
	/// Proof: `AssetManager::SupportedFeePaymentAssets` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetIdType` (r:1 w:1)
	/// Proof: `AssetManager::AssetIdType` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeUnitsPerSecond` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssetManager::AssetTypeId` (r:0 w:1)
	/// Proof: `AssetManager::AssetTypeId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[5, 100]`.
	fn remove_existing_asset_type(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `486 + x * (10 ±0)`
		//  Estimated: `3949 + x * (10 ±0)`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(20_650_636, 0)
			.saturating_add(Weight::from_parts(0, 3949))
			// Standard Error: 5_638
			.saturating_add(Weight::from_parts(331_744, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(x.into()))
	}
}
