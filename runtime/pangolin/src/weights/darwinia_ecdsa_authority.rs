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

//! Autogenerated weights for `darwinia_ecdsa_authority`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `Intel(R) Core(TM) i9-14900KF`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("pangolin-dev")`, DB CACHE: 1024

// Executed Command:
// target/release/darwinia
// benchmark
// pallet
// --header
// .maintain/license-header
// --heap-pages
// 4096
// --chain
// pangolin-dev
// --output
// runtime/pangolin/src/weights
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

/// Weight functions for `darwinia_ecdsa_authority`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> darwinia_ecdsa_authority::WeightInfo for WeightInfo<T> {
	/// Storage: `EcdsaAuthority::AuthoritiesChangeToSign` (r:1 w:0)
	/// Proof: `EcdsaAuthority::AuthoritiesChangeToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::MessageRootToSign` (r:1 w:1)
	/// Proof: `EcdsaAuthority::MessageRootToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::Nonce` (r:1 w:0)
	/// Proof: `EcdsaAuthority::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `154`
		//  Estimated: `1639`
		// Minimum execution time: 8_671_000 picoseconds.
		Weight::from_parts(8_932_000, 0)
			.saturating_add(Weight::from_parts(0, 1639))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EcdsaAuthority::AuthoritiesChangeToSign` (r:1 w:1)
	/// Proof: `EcdsaAuthority::AuthoritiesChangeToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::NextAuthorities` (r:1 w:1)
	/// Proof: `EcdsaAuthority::NextAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::Nonce` (r:1 w:0)
	/// Proof: `EcdsaAuthority::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `154`
		//  Estimated: `1639`
		// Minimum execution time: 11_323_000 picoseconds.
		Weight::from_parts(12_084_000, 0)
			.saturating_add(Weight::from_parts(0, 1639))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `EcdsaAuthority::AuthoritiesChangeToSign` (r:1 w:1)
	/// Proof: `EcdsaAuthority::AuthoritiesChangeToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::NextAuthorities` (r:1 w:1)
	/// Proof: `EcdsaAuthority::NextAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::Nonce` (r:1 w:0)
	/// Proof: `EcdsaAuthority::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `1718`
		// Minimum execution time: 12_434_000 picoseconds.
		Weight::from_parts(13_357_000, 0)
			.saturating_add(Weight::from_parts(0, 1718))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `EcdsaAuthority::AuthoritiesChangeToSign` (r:1 w:1)
	/// Proof: `EcdsaAuthority::AuthoritiesChangeToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::NextAuthorities` (r:1 w:1)
	/// Proof: `EcdsaAuthority::NextAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::Nonce` (r:1 w:0)
	/// Proof: `EcdsaAuthority::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn swap_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `334`
		//  Estimated: `1819`
		// Minimum execution time: 12_841_000 picoseconds.
		Weight::from_parts(13_261_000, 0)
			.saturating_add(Weight::from_parts(0, 1819))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `EcdsaAuthority::Authorities` (r:1 w:1)
	/// Proof: `EcdsaAuthority::Authorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::AuthoritiesChangeToSign` (r:1 w:1)
	/// Proof: `EcdsaAuthority::AuthoritiesChangeToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::NextAuthorities` (r:1 w:0)
	/// Proof: `EcdsaAuthority::NextAuthorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::Nonce` (r:1 w:1)
	/// Proof: `EcdsaAuthority::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::MessageRootToSign` (r:0 w:1)
	/// Proof: `EcdsaAuthority::MessageRootToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn submit_authorities_change_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `318`
		//  Estimated: `1803`
		// Minimum execution time: 40_215_000 picoseconds.
		Weight::from_parts(40_654_000, 0)
			.saturating_add(Weight::from_parts(0, 1803))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `EcdsaAuthority::Authorities` (r:1 w:0)
	/// Proof: `EcdsaAuthority::Authorities` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `EcdsaAuthority::MessageRootToSign` (r:1 w:1)
	/// Proof: `EcdsaAuthority::MessageRootToSign` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn submit_new_message_root_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `312`
		//  Estimated: `1797`
		// Minimum execution time: 30_555_000 picoseconds.
		Weight::from_parts(31_450_000, 0)
			.saturating_add(Weight::from_parts(0, 1797))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
