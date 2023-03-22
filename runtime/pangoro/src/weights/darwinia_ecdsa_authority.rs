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
//! DATE: 2023-03-22, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `inv.cafe`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("pangoro-local"), DB CACHE: 1024

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
// pangoro-local
// --output
// runtime/pangoro/src/weights
// --extrinsic
// *
// --pallet
// darwinia-ecdsa-authority

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `darwinia_ecdsa_authority`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> darwinia_ecdsa_authority::WeightInfo for WeightInfo<T> {
	/// Storage: EcdsaAuthority AuthoritiesChangeToSign (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority AuthoritiesChangeToSign (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority PreviousMessageRoot (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority PreviousMessageRoot (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority Nonce (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority Nonce (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority NewMessageRootToSign (r:0 w:1)
	/// Proof Skipped: EcdsaAuthority NewMessageRootToSign (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `926`
		//  Estimated: `5189`
		// Minimum execution time: 29_889 nanoseconds.
		Weight::from_parts(29_889_000, 5189)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: EcdsaAuthority AuthoritiesChangeToSign (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority AuthoritiesChangeToSign (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority NextAuthorities (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority NextAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority Nonce (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority Nonce (max_values: Some(1), max_size: None, mode: Measured)
	fn add_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1103`
		//  Estimated: `4794`
		// Minimum execution time: 39_171 nanoseconds.
		Weight::from_parts(39_171_000, 4794)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: EcdsaAuthority AuthoritiesChangeToSign (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority AuthoritiesChangeToSign (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority NextAuthorities (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority NextAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority Nonce (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority Nonce (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1214`
		//  Estimated: `5127`
		// Minimum execution time: 24_106 nanoseconds.
		Weight::from_parts(24_106_000, 5127)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: EcdsaAuthority AuthoritiesChangeToSign (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority AuthoritiesChangeToSign (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority NextAuthorities (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority NextAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority Nonce (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority Nonce (max_values: Some(1), max_size: None, mode: Measured)
	fn swap_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1234`
		//  Estimated: `5187`
		// Minimum execution time: 23_658 nanoseconds.
		Weight::from_parts(23_658_000, 5187)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: EcdsaAuthority Authorities (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority Authorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority AuthoritiesChangeToSign (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority AuthoritiesChangeToSign (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority NextAuthorities (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority NextAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority Nonce (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority Nonce (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority PreviousMessageRoot (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority PreviousMessageRoot (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority NewMessageRootToSign (r:0 w:1)
	/// Proof Skipped: EcdsaAuthority NewMessageRootToSign (max_values: Some(1), max_size: None, mode: Measured)
	fn submit_authorities_change_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2254`
		//  Estimated: `18831`
		// Minimum execution time: 62_099 nanoseconds.
		Weight::from_parts(62_099_000, 18831)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: EcdsaAuthority Authorities (r:1 w:0)
	/// Proof Skipped: EcdsaAuthority Authorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: EcdsaAuthority MessageRootToSign (r:1 w:1)
	/// Proof Skipped: EcdsaAuthority MessageRootToSign (max_values: Some(1), max_size: None, mode: Measured)
	fn submit_new_message_root_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1292`
		//  Estimated: `3574`
		// Minimum execution time: 24_637 nanoseconds.
		Weight::from_parts(24_637_000, 3574)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
