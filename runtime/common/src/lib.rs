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
// TODO:
// #![deny(missing_docs)]

pub mod gov_origin;
pub mod xcm_configs;

pub use bp_darwinia_core as bp_crab;
pub use bp_darwinia_core as bp_darwinia;
pub use bp_darwinia_core as bp_pangolin;

#[cfg(feature = "test")]
pub mod test;

// darwinia
use dc_primitives::*;
// substrate
use frame_support::{
	sp_runtime::Perbill,
	weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
};

#[macro_export]
macro_rules! fast_runtime_or_not {
	($name:ident, $development_type:ty, $production_type:ty) => {
		#[cfg(feature = "fast-runtime")]
		type $name = $development_type;
		#[cfg(not(feature = "fast-runtime"))]
		type $name = $production_type;
	};
}

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
		// here, we map to 1/10 of that, or 1/10 MILLIUNIT
		let p = MILLIUNIT / 10;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// Deposit calculator for Darwinia.
/// 100 UNIT for the base fee, 102.4 UNIT/MB.
pub const fn darwinia_deposit(items: u32, bytes: u32) -> Balance {
	// First try.
	items as Balance * 100 * UNIT + (bytes as Balance) * 100 * MICROUNIT
	// items as Balance * 100 * UNIT + (bytes as Balance) * 100 * MILLIUNIT
}

#[macro_export]
macro_rules! impl_self_contained_call {
	() => {
		impl fp_self_contained::SelfContainedCall for RuntimeCall {
			type SignedInfo = H160;

			fn is_self_contained(&self) -> bool {
				match self {
					RuntimeCall::Ethereum(call) => call.is_self_contained(),
					_ => false,
				}
			}

			fn check_self_contained(
				&self,
			) -> Option<
				Result<
					Self::SignedInfo,
					sp_runtime::transaction_validity::TransactionValidityError,
				>,
			> {
				match self {
					RuntimeCall::Ethereum(call) => call.check_self_contained(),
					_ => None,
				}
			}

			fn validate_self_contained(
				&self,
				info: &Self::SignedInfo,
				dispatch_info: &sp_runtime::traits::DispatchInfoOf<RuntimeCall>,
				len: usize,
			) -> Option<TransactionValidity> {
				match self {
					RuntimeCall::Ethereum(call) =>
						call.validate_self_contained(info, dispatch_info, len),
					_ => None,
				}
			}

			fn pre_dispatch_self_contained(
				&self,
				info: &Self::SignedInfo,
				dispatch_info: &sp_runtime::traits::DispatchInfoOf<RuntimeCall>,
				len: usize,
			) -> Option<Result<(), sp_runtime::transaction_validity::TransactionValidityError>> {
				match self {
					RuntimeCall::Ethereum(call) =>
						call.pre_dispatch_self_contained(info, dispatch_info, len),
					_ => None,
				}
			}

			fn apply_self_contained(
				self,
				info: Self::SignedInfo,
			) -> Option<
				sp_runtime::DispatchResultWithInfo<sp_runtime::traits::PostDispatchInfoOf<Self>>,
			> {
				// substrate
				use sp_runtime::traits::Dispatchable;

				match self {
					call @ RuntimeCall::Ethereum(pallet_ethereum::Call::transact { .. }) =>
						Some(call.dispatch(RuntimeOrigin::from(
							pallet_ethereum::RawOrigin::EthereumTransaction(info),
						))),
					_ => None,
				}
			}
		}
	};
}
