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

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-23, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("pangolin-local"), DB CACHE: 1024

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
// pangolin-local
// --output
// runtime/pangolin/src/weights
// --extrinsic
// *
// --pallet
// *

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: TechnicalCommittee Members (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:100 w:100)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (2048 ±0) + p * (2027 ±0)`
		//  Estimated: `3293 + m * (563689 ±21_593_704_177_905_168_285_696) + p * (3651 ±286)`
		// Minimum execution time: 13_336_000 picoseconds.
		Weight::from_parts(13_336_000, 0)
			.saturating_add(Weight::from_parts(0, 3293))
			// Standard Error: 951_754
			.saturating_add(Weight::from_parts(3_774_130, 0).saturating_mul(m.into()))
			// Standard Error: 951_754
			.saturating_add(Weight::from_parts(3_740_020, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 563689).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 3651).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(_b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100 + m * (20 ±0)`
		//  Estimated: `1586 + m * (20 ±0)`
		// Minimum execution time: 13_694_000 picoseconds.
		Weight::from_parts(14_459_709, 0)
			.saturating_add(Weight::from_parts(0, 1586))
			// Standard Error: 10_170
			.saturating_add(Weight::from_parts(11_555, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 20).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100 + m * (20 ±0)`
		//  Estimated: `5152 + m * (40 ±0)`
		// Minimum execution time: 16_255_000 picoseconds.
		Weight::from_parts(15_090_229, 0)
			.saturating_add(Weight::from_parts(0, 5152))
			// Standard Error: 197
			.saturating_add(Weight::from_parts(1_364, 0).saturating_mul(b.into()))
			// Standard Error: 2_035
			.saturating_add(Weight::from_parts(13_040, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37 + m * (20 ±0) + p * (39 ±0)`
		//  Estimated: `8105 + m * (100 ±0) + p * (195 ±0)`
		// Minimum execution time: 21_608_000 picoseconds.
		Weight::from_parts(21_202_251, 0)
			.saturating_add(Weight::from_parts(0, 8105))
			// Standard Error: 1_563
			.saturating_add(Weight::from_parts(754, 0).saturating_mul(b.into()))
			// Standard Error: 16_306
			.saturating_add(Weight::from_parts(9_882, 0).saturating_mul(m.into()))
			// Standard Error: 16_141
			.saturating_add(Weight::from_parts(157_974, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 100).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 195).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `865 + m * (40 ±0)`
		//  Estimated: `6682 + m * (80 ±0)`
		// Minimum execution time: 23_635_000 picoseconds.
		Weight::from_parts(24_126_578, 0)
			.saturating_add(Weight::from_parts(0, 6682))
			// Standard Error: 16_699
			.saturating_add(Weight::from_parts(11_184, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(m.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `227 + m * (40 ±0) + p * (38 ±0)`
		//  Estimated: `7343 + m * (160 ±0) + p * (156 ±0)`
		// Minimum execution time: 21_718_000 picoseconds.
		Weight::from_parts(20_126_764, 0)
			.saturating_add(Weight::from_parts(0, 7343))
			// Standard Error: 13_716
			.saturating_add(Weight::from_parts(19_484, 0).saturating_mul(m.into()))
			// Standard Error: 13_300
			.saturating_add(Weight::from_parts(176_797, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 160).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 156).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `442 + m * (40 ±0) + p * (44 ±0)`
		//  Estimated: `11668 + b * (4 ±0) + m * (160 ±0) + p * (180 ±0)`
		// Minimum execution time: 31_294_000 picoseconds.
		Weight::from_parts(21_707_241, 0)
			.saturating_add(Weight::from_parts(0, 11668))
			// Standard Error: 1_353
			.saturating_add(Weight::from_parts(5_407, 0).saturating_mul(b.into()))
			// Standard Error: 14_406
			.saturating_add(Weight::from_parts(41_958, 0).saturating_mul(m.into()))
			// Standard Error: 13_970
			.saturating_add(Weight::from_parts(255_611, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 4).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 160).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 180).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246 + m * (40 ±0) + p * (38 ±0)`
		//  Estimated: `9150 + m * (200 ±0) + p * (195 ±0)`
		// Minimum execution time: 30_724_000 picoseconds.
		Weight::from_parts(30_042_142, 0)
			.saturating_add(Weight::from_parts(0, 9150))
			// Standard Error: 25_194
			.saturating_add(Weight::from_parts(7_364, 0).saturating_mul(m.into()))
			// Standard Error: 24_431
			.saturating_add(Weight::from_parts(124_398, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 200).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 195).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Members (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Prime (r:1 w:0)
	/// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `461 + m * (40 ±0) + p * (44 ±0)`
		//  Estimated: `13690 + b * (5 ±0) + m * (200 ±0) + p * (225 ±0)`
		// Minimum execution time: 35_871_000 picoseconds.
		Weight::from_parts(33_699_013, 0)
			.saturating_add(Weight::from_parts(0, 13690))
			// Standard Error: 1_287
			.saturating_add(Weight::from_parts(1_683, 0).saturating_mul(b.into()))
			// Standard Error: 13_711
			.saturating_add(Weight::from_parts(4_109, 0).saturating_mul(m.into()))
			// Standard Error: 13_296
			.saturating_add(Weight::from_parts(219_181, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 5).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 200).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 225).saturating_mul(p.into()))
	}
	/// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258 + p * (32 ±0)`
		//  Estimated: `2262 + p * (96 ±0)`
		// Minimum execution time: 13_449_000 picoseconds.
		Weight::from_parts(14_483_020, 0)
			.saturating_add(Weight::from_parts(0, 2262))
			// Standard Error: 16_349
			.saturating_add(Weight::from_parts(143_979, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 96).saturating_mul(p.into()))
	}
}
