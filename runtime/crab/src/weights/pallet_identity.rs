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

//! Autogenerated weights for `pallet_identity`
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `30 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 21_809_000 picoseconds.
		Weight::from_parts(25_348_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + r * (5 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 38_256_000 picoseconds.
		Weight::from_parts(37_386_526, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 133_003
			.saturating_add(Weight::from_parts(43_473, 0).saturating_mul(r.into()))
			// Standard Error: 25_270
			.saturating_add(Weight::from_parts(442_410, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:100 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89`
		//  Estimated: `273992`
		// Minimum execution time: 19_152_000 picoseconds.
		Weight::from_parts(271_044_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(102))
			.saturating_add(T::DbWeight::get().writes(101))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + p * (20 ±0)`
		//  Estimated: `16502`
		// Minimum execution time: 19_147_000 picoseconds.
		Weight::from_parts(122_675_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(101))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `364 + r * (5 ±0) + s * (20 ±0) + x * (66 ±0)`
		//  Estimated: `16502`
		// Minimum execution time: 56_848_000 picoseconds.
		Weight::from_parts(27_829_578, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 196_924
			.saturating_add(Weight::from_parts(285_421, 0).saturating_mul(r.into()))
			// Standard Error: 37_415
			.saturating_add(Weight::from_parts(967_350, 0).saturating_mul(s.into()))
			// Standard Error: 37_415
			.saturating_add(Weight::from_parts(233_100, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `352 + r * (45 ±0) + x * (66 ±0)`
		//  Estimated: `13377`
		// Minimum execution time: 45_482_000 picoseconds.
		Weight::from_parts(36_518_842, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 576_225
			.saturating_add(Weight::from_parts(448_157, 0).saturating_mul(r.into()))
			// Standard Error: 109_482
			.saturating_add(Weight::from_parts(413_860, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(_r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384 + x * (66 ±0)`
		//  Estimated: `10991`
		// Minimum execution time: 41_189_000 picoseconds.
		Weight::from_parts(45_514_789, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2_987
			.saturating_add(Weight::from_parts(368_585, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 15_762_000 picoseconds.
		Weight::from_parts(17_119_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 18_780_000 picoseconds.
		Weight::from_parts(19_381_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `77 + r * (45 ±0)`
		//  Estimated: `2386`
		// Minimum execution time: 15_608_000 picoseconds.
		Weight::from_parts(16_252_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(901), added: 1396, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(_r: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419 + r * (45 ±0) + x * (66 ±0)`
		//  Estimated: `13377`
		// Minimum execution time: 29_930_000 picoseconds.
		Weight::from_parts(43_912_944, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 10_963
			.saturating_add(Weight::from_parts(674_480, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(116), added: 2591, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(_r: u32, s: u32, x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `614 + r * (10 ±0) + s * (20 ±0) + x * (66 ±0)`
		//  Estimated: `22674`
		// Minimum execution time: 71_946_000 picoseconds.
		Weight::from_parts(85_841_105, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 102_721
			.saturating_add(Weight::from_parts(1_037_210, 0).saturating_mul(s.into()))
			// Standard Error: 102_721
			.saturating_add(Weight::from_parts(289_470, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `89 + s * (27 ±0)`
		//  Estimated: `20057`
		// Minimum execution time: 38_035_000 picoseconds.
		Weight::from_parts(44_867_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229 + s * (7 ±0)`
		//  Estimated: `14546`
		// Minimum execution time: 24_147_000 picoseconds.
		Weight::from_parts(24_454_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7526), added: 10001, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264 + s * (27 ±0)`
		//  Estimated: `20057`
		// Minimum execution time: 42_514_000 picoseconds.
		Weight::from_parts(51_696_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(2046), added: 4521, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `244 + s * (27 ±0)`
		//  Estimated: `9066`
		// Minimum execution time: 34_071_000 picoseconds.
		Weight::from_parts(34_317_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
