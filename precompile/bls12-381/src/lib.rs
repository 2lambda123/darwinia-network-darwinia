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

#![cfg_attr(not(feature = "std"), no_std)]

mod bls;
use bls::{hash_to_curve_g2, PublicKey, Signature};

// core
use core::marker::PhantomData;
// frontier
use pallet_evm::GasWeightMapping;
// moonbeam
use precompile_utils::prelude::*;
// substrate
use frame_support::weights::Weight;
use sp_std::prelude::*;

/// The BLS verification is a computationally intensive process. Normally, it consumes a lot of
/// block weight according to our benchmark test. Tested on the `AMD Ryzen 7 5700G`,  this
/// precompile consumed at least 117_954_459_000 weight. So we give them more than that to ensure
/// there is enough time for other machine types.
pub(crate) const BLS_BENCHMARKED_WEIGHT: u64 = 150_000_000_000;
pub struct BLS12381<T>(PhantomData<T>);

#[precompile_utils::precompile]
impl<Runtime: pallet_evm::Config> BLS12381<Runtime> {
	#[precompile::public("fast_aggregate_verify(bytes[],bytes,bytes)")]
	#[precompile::view]
	fn fast_aggregate_verify(
		handle: &mut impl PrecompileHandle,
		pubkeys: Vec<UnboundedBytes>,
		message: UnboundedBytes,
		signature: UnboundedBytes,
	) -> EvmResult<bool> {
		handle.record_cost(<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
			Weight::from_ref_time(BLS_BENCHMARKED_WEIGHT),
		))?;

		let asig =
			Signature::from_bytes(signature.as_bytes()).map_err(|_| revert("Invalid signature"))?;
		let public_keys: Result<Vec<PublicKey>, _> =
			pubkeys.into_iter().map(|k| PublicKey::from_bytes(k.as_bytes())).collect();
		let Ok(pks) = public_keys else {
            return Err(revert("Invalid pubkeys"));
        };

		let apk = PublicKey::aggregate(pks);
		let msg = hash_to_curve_g2(message.as_bytes()).map_err(|_| revert("Invalid message"))?;
		let result = apk.verify(&asig, &msg);
		Ok(result)
	}
}
