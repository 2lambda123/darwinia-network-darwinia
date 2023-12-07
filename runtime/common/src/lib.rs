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
#![deny(unused_crate_dependencies)]
// TODO:
// #![deny(missing_docs)]

pub mod gov_origin;
pub mod xcm_configs;

pub use bp_darwinia_core as bp_crab;
pub use bp_darwinia_core as bp_darwinia;
pub use bp_darwinia_core as bp_pangolin;
pub use bp_darwinia_core as bp_pangoro;

#[cfg(feature = "test")]
pub mod test;

// darwinia
use dc_primitives::*;
// substrate
use frame_support::{
	sp_runtime::Perbill,
	weights::{
		constants::ExtrinsicBaseWeight, FeePolynomial, Weight, WeightToFee as WeightToFeeT,
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	},
};

#[macro_export]
macro_rules! fast_runtime_or_not {
	($name:ident, $type:ty, $fast:expr, $regular:expr) => {
		#[cfg(feature = "fast-runtime")]
		const $name: $type = $fast;
		#[cfg(not(feature = "fast-runtime"))]
		const $name: $type = $regular;
	};
}

#[macro_export]
macro_rules! impl_self_contained_call {
	() => {
		impl fp_self_contained::SelfContainedCall for RuntimeCall {
			type SignedInfo = sp_core::H160;

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
			) -> Option<sp_runtime::transaction_validity::TransactionValidity> {
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

#[cfg(not(feature = "runtime-benchmarks"))]
pub const EXISTENTIAL_DEPOSIT: Balance = 0;
#[cfg(feature = "runtime-benchmarks")]
pub const EXISTENTIAL_DEPOSIT: Balance = 1;

/// Darwinia proposal base fee.
pub const DARWINIA_PROPOSAL_REQUIREMENT: Balance = 5_000 * UNIT;

/// Deposit calculator for Darwinia.
/// 100 UNIT for the base fee, 102.4 UNIT/MB.
pub const fn darwinia_deposit(items: u32, bytes: u32) -> Balance {
	// First try.
	items as Balance * 100 * UNIT + (bytes as Balance) * 100 * MICROUNIT
	// items as Balance * 100 * UNIT + (bytes as Balance) * 100 * MILLIUNIT
}

/// Construct a [`FixedI64`] percent quickly.
pub const fn percent(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 100)
}

/// Construct a [`FixedI64`] permill quickly.
pub const fn permill(x: i32) -> sp_runtime::FixedI64 {
	sp_runtime::FixedI64::from_rational(x as u128, 1000)
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
impl WeightToFeeT for WeightToFee {
	type Balance = Balance;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		let time_poly: FeePolynomial<Balance> = RefTimeToFee::polynomial().into();
		let proof_poly: FeePolynomial<Balance> = ProofSizeToFee::polynomial().into();

		// Take the maximum instead of the sum to charge by the more scarce resource.
		time_poly.eval(weight.ref_time()).max(proof_poly.eval(weight.proof_size()))
	}
}

/// Maps the reference time component of `Weight` to a fee.
pub struct RefTimeToFee;
impl WeightToFeePolynomial for RefTimeToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// Map base extrinsic weight to 1/100 UNIT.
		let p = UNIT;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());

		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// Maps the proof size component of `Weight` to a fee.
pub struct ProofSizeToFee;
impl WeightToFeePolynomial for ProofSizeToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// Map 10kb proof to 1 UNIT.
		let p = UNIT;
		let q = 10_000;

		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// EIP-1559 like configuration.
///
/// Burn the base fee and allocate tips to the block producer.
pub struct DealWithFees<R>(core::marker::PhantomData<R>);
impl<R> frame_support::traits::OnUnbalanced<pallet_balances::NegativeImbalance<R>>
	for DealWithFees<R>
where
	R: pallet_authorship::Config + pallet_balances::Config,
{
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<R>>,
	) {
		// substrate
		use frame_support::traits::Currency;

		if fees_then_tips.next().is_some() {
			if let Some(tips) = fees_then_tips.next() {
				if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
					// Tip the block producer here.
					<pallet_balances::Pallet<R>>::resolve_creating(&author, tips);
				}

				// Burn the tips here. (IMPOSSIBLE CASE)
			}
		}

		// Burn the base fee here.
	}
}

pub struct FindAuthor<Inner>(core::marker::PhantomData<Inner>);
impl<Inner> frame_support::traits::FindAuthor<sp_core::H160> for FindAuthor<Inner>
where
	Inner: frame_support::traits::FindAuthor<AccountId>,
{
	fn find_author<'a, I>(digests: I) -> Option<sp_core::H160>
	where
		I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
	{
		Inner::find_author(digests).map(Into::into)
	}
}

pub struct FixedGasPrice;
impl pallet_evm::FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (sp_core::U256, frame_support::weights::Weight) {
		(sp_core::U256::from(GWEI), frame_support::weights::Weight::zero())
	}
}

pub struct AssetIdConverter;
impl darwinia_precompile_assets::AccountToAssetId<AccountId, AssetId> for AssetIdConverter {
	fn account_to_asset_id(account_id: AccountId) -> AssetId {
		let addr: sp_core::H160 = account_id.into();
		addr.to_low_u64_be()
	}
}
/// Helper for pallet-assets benchmarking.
#[cfg(feature = "runtime-benchmarks")]
pub struct AssetsBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_assets::BenchmarkHelper<codec::Compact<u64>> for AssetsBenchmarkHelper {
	fn create_asset_id_parameter(id: u32) -> codec::Compact<u64> {
		u64::from(id).into()
	}
}
