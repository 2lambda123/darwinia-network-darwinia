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

//! Autogenerated weights for darwinia_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("koi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --template
// .maintain/pallet-weight-template.hbs
// --heap-pages
// 4096
// --chain
// koi-dev
// --output
// pallet/staking/src/weights.rs
// --extrinsic
// *
// --pallet
// darwinia-staking

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for darwinia_staking.
pub trait WeightInfo {
	fn stake(x: u32, ) -> Weight;
	fn unstake(x: u32, ) -> Weight;
	fn collect() -> Weight;
	fn nominate() -> Weight;
	fn chill() -> Weight;
	fn payout() -> Weight;
	fn set_rate_limit() -> Weight;
	fn set_kton_reward_distribution_contract() -> Weight;
	fn set_collator_count() -> Weight;
}

/// Weights for darwinia_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1078), added: 3553, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimitState` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RateLimitState` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimit` (r:1 w:0)
	/// Proof: `DarwiniaStaking::RateLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn stake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7088 + x * (25 ±0)`
		//  Estimated: `29615`
		// Minimum execution time: 60_000 nanoseconds.
		Weight::from_parts(992_585_597, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 297_913
			.saturating_add(Weight::from_parts(13_623_433, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1078), added: 3553, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimitState` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RateLimitState` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimit` (r:1 w:0)
	/// Proof: `DarwiniaStaking::RateLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn unstake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7415 + x * (26 ±0)`
		//  Estimated: `29615`
		// Minimum execution time: 54_000 nanoseconds.
		Weight::from_parts(908_132_841, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 231_565
			.saturating_add(Weight::from_parts(11_600_487, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn collect() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3497`
		// Minimum execution time: 11_000 nanoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(Weight::from_parts(3497, 0))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1078), added: 3553, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Nominators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nominate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `574`
		//  Estimated: `4543`
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(4543, 0))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Nominators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_000 nanoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DarwiniaStaking::ExposureCacheStates` (r:1 w:0)
	/// Proof: `DarwiniaStaking::ExposureCacheStates` (`max_values`: Some(1), `max_size`: Some(3), added: 498, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::ExposureCache2` (r:1 w:1)
	/// Proof: `DarwiniaStaking::ExposureCache2` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DarwiniaStaking::PendingRewards` (r:1 w:1)
	/// Proof: `DarwiniaStaking::PendingRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1845`
		//  Estimated: `5310`
		// Minimum execution time: 205_000 nanoseconds.
		Weight::from_parts(207_000_000, 0)
			.saturating_add(Weight::from_parts(5310, 0))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `DarwiniaStaking::RateLimit` (r:0 w:1)
	/// Proof: `DarwiniaStaking::RateLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `DarwiniaStaking::KtonRewardDistributionContract` (r:0 w:1)
	/// Proof: `DarwiniaStaking::KtonRewardDistributionContract` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_kton_reward_distribution_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::CollatorCount` (r:0 w:1)
	/// Proof: `DarwiniaStaking::CollatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_collator_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1078), added: 3553, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimitState` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RateLimitState` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimit` (r:1 w:0)
	/// Proof: `DarwiniaStaking::RateLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn stake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7088 + x * (25 ±0)`
		//  Estimated: `29615`
		// Minimum execution time: 60_000 nanoseconds.
		Weight::from_parts(992_585_597, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 297_913
			.saturating_add(Weight::from_parts(13_623_433, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1078), added: 3553, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Deposit::Deposits` (r:1 w:1)
	/// Proof: `Deposit::Deposits` (`max_values`: None, `max_size`: Some(26150), added: 28625, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimitState` (r:1 w:1)
	/// Proof: `DarwiniaStaking::RateLimitState` (`max_values`: Some(1), `max_size`: Some(17), added: 512, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::RateLimit` (r:1 w:0)
	/// Proof: `DarwiniaStaking::RateLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 1023]`.
	fn unstake(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7415 + x * (26 ±0)`
		//  Estimated: `29615`
		// Minimum execution time: 54_000 nanoseconds.
		Weight::from_parts(908_132_841, 0)
			.saturating_add(Weight::from_parts(29615, 0))
			// Standard Error: 231_565
			.saturating_add(Weight::from_parts(11_600_487, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn collect() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `3497`
		// Minimum execution time: 11_000 nanoseconds.
		Weight::from_parts(11_000_000, 0)
			.saturating_add(Weight::from_parts(3497, 0))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Ledgers` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Ledgers` (`max_values`: None, `max_size`: Some(1078), added: 3553, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:1 w:0)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Nominators` (r:1 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nominate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `574`
		//  Estimated: `4543`
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_parts(12_000_000, 0)
			.saturating_add(Weight::from_parts(4543, 0))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::Nominators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Nominators` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::Collators` (r:0 w:1)
	/// Proof: `DarwiniaStaking::Collators` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_000 nanoseconds.
		Weight::from_parts(4_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `DarwiniaStaking::ExposureCacheStates` (r:1 w:0)
	/// Proof: `DarwiniaStaking::ExposureCacheStates` (`max_values`: Some(1), `max_size`: Some(3), added: 498, mode: `MaxEncodedLen`)
	/// Storage: `DarwiniaStaking::ExposureCache2` (r:1 w:1)
	/// Proof: `DarwiniaStaking::ExposureCache2` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `DarwiniaStaking::PendingRewards` (r:1 w:1)
	/// Proof: `DarwiniaStaking::PendingRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1845`
		//  Estimated: `5310`
		// Minimum execution time: 205_000 nanoseconds.
		Weight::from_parts(207_000_000, 0)
			.saturating_add(Weight::from_parts(5310, 0))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `DarwiniaStaking::RateLimit` (r:0 w:1)
	/// Proof: `DarwiniaStaking::RateLimit` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::KtonRewardDistributionContract` (r:0 w:1)
	/// Proof: `DarwiniaStaking::KtonRewardDistributionContract` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn set_kton_reward_distribution_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `DarwiniaStaking::CollatorCount` (r:0 w:1)
	/// Proof: `DarwiniaStaking::CollatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_collator_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000 nanoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
