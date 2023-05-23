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

//! Autogenerated weights for `pallet_democracy`
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

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: Democracy PublicPropCount (r:1 w:1)
	/// Proof: Democracy PublicPropCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:0 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3844`
		//  Estimated: `23979`
		// Minimum execution time: 37_997_000 picoseconds.
		Weight::from_parts(37_997_000, 0)
			.saturating_add(Weight::from_parts(0, 23979))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2593`
		//  Estimated: `5495`
		// Minimum execution time: 30_983_000 picoseconds.
		Weight::from_parts(30_983_000, 0)
			.saturating_add(Weight::from_parts(0, 5495))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3440`
		//  Estimated: `15666`
		// Minimum execution time: 60_878_000 picoseconds.
		Weight::from_parts(60_878_000, 0)
			.saturating_add(Weight::from_parts(0, 15666))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3462`
		//  Estimated: `15666`
		// Minimum execution time: 61_143_000 picoseconds.
		Weight::from_parts(61_143_000, 0)
			.saturating_add(Weight::from_parts(0, 15666))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Cancellations (r:1 w:1)
	/// Proof: Democracy Cancellations (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `333`
		//  Estimated: `10682`
		// Minimum execution time: 39_595_000 picoseconds.
		Weight::from_parts(39_595_000, 0)
			.saturating_add(Weight::from_parts(0, 10682))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:3 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:0 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4954`
		//  Estimated: `42511`
		// Minimum execution time: 115_843_000 picoseconds.
		Weight::from_parts(115_843_000, 0)
			.saturating_add(Weight::from_parts(0, 42511))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2195`
		//  Estimated: `7120`
		// Minimum execution time: 29_698_000 picoseconds.
		Weight::from_parts(29_698_000, 0)
			.saturating_add(Weight::from_parts(0, 7120))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 25_758_000 picoseconds.
		Weight::from_parts(25_758_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_904_000 picoseconds.
		Weight::from_parts(15_904_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:1)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:2)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `6624`
		// Minimum execution time: 36_786_000 picoseconds.
		Weight::from_parts(36_786_000, 0)
			.saturating_add(Weight::from_parts(0, 6624))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(2038), added: 4513, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2298`
		//  Estimated: `10638`
		// Minimum execution time: 45_406_000 picoseconds.
		Weight::from_parts(45_406_000, 0)
			.saturating_add(Weight::from_parts(0, 10638))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(2030), added: 4505, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4886`
		//  Estimated: `32172`
		// Minimum execution time: 85_869_000 picoseconds.
		Weight::from_parts(85_869_000, 0)
			.saturating_add(Weight::from_parts(0, 32172))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `3518`
		// Minimum execution time: 32_531_000 picoseconds.
		Weight::from_parts(32_531_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150 + r * (87 ±0)`
		//  Estimated: `268892`
		// Minimum execution time: 16_322_000 picoseconds.
		Weight::from_parts(223_260_000, 0)
			.saturating_add(Weight::from_parts(0, 268892))
			.saturating_add(T::DbWeight::get().reads(101))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	/// Proof: Democracy LastTabledWasExternal (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150 + r * (87 ±0)`
		//  Estimated: `288982`
		// Minimum execution time: 20_661_000 picoseconds.
		Weight::from_parts(261_488_000, 0)
			.saturating_add(Weight::from_parts(0, 288982))
			.saturating_add(T::DbWeight::get().reads(104))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy VotingOf (r:3 w:3)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `603 + r * (110 ±0)`
		//  Estimated: `290430`
		// Minimum execution time: 36_519_000 picoseconds.
		Weight::from_parts(365_075_000, 0)
			.saturating_add(Weight::from_parts(0, 290430))
			.saturating_add(T::DbWeight::get().reads(103))
			.saturating_add(T::DbWeight::get().writes(103))
	}
	/// Storage: Democracy VotingOf (r:2 w:2)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `387 + r * (109 ±0)`
		//  Estimated: `279420`
		// Minimum execution time: 37_194_000 picoseconds.
		Weight::from_parts(337_967_000, 0)
			.saturating_add(Weight::from_parts(0, 279420))
			.saturating_add(T::DbWeight::get().reads(101))
			.saturating_add(T::DbWeight::get().writes(101))
	}
	/// Storage: Democracy PublicProps (r:0 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_416_000 picoseconds.
		Weight::from_parts(12_416_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241 + r * (1 ±0)`
		//  Estimated: `15581`
		// Minimum execution time: 32_846_000 picoseconds.
		Weight::from_parts(39_992_000, 0)
			.saturating_add(Weight::from_parts(0, 15581))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419 + r * (22 ±0)`
		//  Estimated: `15581`
		// Minimum execution time: 38_005_000 picoseconds.
		Weight::from_parts(44_328_000, 0)
			.saturating_add(Weight::from_parts(0, 15581))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `367 + r * (28 ±0)`
		//  Estimated: `10914`
		// Minimum execution time: 28_996_000 picoseconds.
		Weight::from_parts(37_605_000, 0)
			.saturating_add(Weight::from_parts(0, 10914))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3783), added: 6258, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `367 + r * (28 ±0)`
		//  Estimated: `10914`
		// Minimum execution time: 26_335_000 picoseconds.
		Weight::from_parts(40_813_000, 0)
			.saturating_add(Weight::from_parts(0, 10914))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `323`
		//  Estimated: `5161`
		// Minimum execution time: 29_978_000 picoseconds.
		Weight::from_parts(29_978_000, 0)
			.saturating_add(Weight::from_parts(0, 5161))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `5135`
		// Minimum execution time: 24_607_000 picoseconds.
		Weight::from_parts(24_607_000, 0)
			.saturating_add(Weight::from_parts(0, 5135))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3655`
		//  Estimated: `20531`
		// Minimum execution time: 51_346_000 picoseconds.
		Weight::from_parts(51_346_000, 0)
			.saturating_add(Weight::from_parts(0, 20531))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(15502), added: 15997, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3589`
		//  Estimated: `20505`
		// Minimum execution time: 46_515_000 picoseconds.
		Weight::from_parts(46_515_000, 0)
			.saturating_add(Weight::from_parts(0, 20505))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3544`
		// Minimum execution time: 28_377_000 picoseconds.
		Weight::from_parts(28_377_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `7184`
		// Minimum execution time: 33_593_000 picoseconds.
		Weight::from_parts(33_593_000, 0)
			.saturating_add(Weight::from_parts(0, 7184))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
