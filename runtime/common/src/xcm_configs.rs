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

// core
use core::marker::PhantomData;
// crates.io
use codec::Encode;
// darwinia
use dc_primitives::GWEI;
// polkadot
use xcm::latest::{prelude::*, Weight as XcmWeight};
use xcm_builder::TakeRevenue;
use xcm_executor::{
	traits::{Convert, ShouldExecute, WeightTrader},
	Assets,
};
// substrate
use frame_support::{
	log,
	traits::{
		tokens::currency::Currency as CurrencyT, ConstU128, OnUnbalanced as OnUnbalancedT,
		ProcessMessageError,
	},
	weights::{Weight, WeightToFee as WeightToFeeT},
};
use sp_core::Get;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::{SaturatedConversion, Saturating, Zero};
use sp_std::{borrow::Borrow, result::Result};

/// Base balance required for the XCM unit weight.
pub type XcmBaseWeightFee = ConstU128<GWEI>;

frame_support::match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

//TODO: move DenyThenTry to polkadot's xcm module.
/// Deny executing the xcm message if it matches any of the Deny filter regardless of anything else.
/// If it passes the Deny, and matches one of the Allow cases then it is let through.
pub struct DenyThenTry<Deny, Allow>(PhantomData<Deny>, PhantomData<Allow>)
where
	Deny: ShouldExecute,
	Allow: ShouldExecute;

impl<Deny, Allow> ShouldExecute for DenyThenTry<Deny, Allow>
where
	Deny: ShouldExecute,
	Allow: ShouldExecute,
{
	fn should_execute<RuntimeCall>(
		origin: &MultiLocation,
		message: &mut [Instruction<RuntimeCall>],
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ProcessMessageError> {
		Deny::should_execute(origin, message, max_weight, weight_credit)?;
		Allow::should_execute(origin, message, max_weight, weight_credit)
	}
}

// See issue <https://github.com/paritytech/polkadot/issues/5233>
pub struct DenyReserveTransferToRelayChain;
impl ShouldExecute for DenyReserveTransferToRelayChain {
	fn should_execute<RuntimeCall>(
		origin: &MultiLocation,
		message: &mut [Instruction<RuntimeCall>],
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ProcessMessageError> {
		if message.iter().any(|inst| {
			matches!(
				inst,
				InitiateReserveWithdraw {
					reserve: MultiLocation { parents: 1, interior: Here },
					..
				} | DepositReserveAsset { dest: MultiLocation { parents: 1, interior: Here }, .. }
					| TransferReserveAsset {
						dest: MultiLocation { parents: 1, interior: Here },
						..
					}
			)
		}) {
			return Err(ProcessMessageError::Unsupported); // Deny
		}

		// An unexpected reserve transfer has arrived from the Relay Chain. Generally, `IsReserve`
		// should not allow this, but we just log it here.
		if matches!(origin, MultiLocation { parents: 1, interior: Here })
			&& message.iter().any(|inst| matches!(inst, ReserveAssetDeposited { .. }))
		{
			log::warn!(
				target: "xcm::barriers",
				"Unexpected ReserveAssetDeposited from the Relay Chain",
			);
		}
		// Permit everything else
		Ok(())
	}
}

/// Struct that converts a given MultiLocation into a 20 bytes account id by hashing
/// with blake2_256 and taking the first 20 bytes
pub struct Account20Hash<AccountId>(PhantomData<AccountId>);
impl<AccountId: From<[u8; 20]> + Into<[u8; 20]> + Clone> Convert<MultiLocation, AccountId>
	for Account20Hash<AccountId>
{
	fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
		let hash: [u8; 32] = ("multiloc", location.borrow()).using_encoded(blake2_256);
		let mut account_id = [0u8; 20];
		account_id.copy_from_slice(&hash[0..20]);
		Ok(account_id.into())
	}

	fn reverse_ref(_: impl Borrow<AccountId>) -> Result<MultiLocation, ()> {
		Err(())
	}
}

/// Weight trader to set the right price for weight and then places any weight bought into the right
/// account. Refer to: https://github.com/paritytech/polkadot/blob/release-v0.9.30/xcm/xcm-builder/src/weight.rs#L242-L305
pub struct LocalAssetTrader<
	WeightToFee: WeightToFeeT<Balance = Currency::Balance>,
	AssetId: Get<MultiLocation>,
	AccountId,
	Currency: CurrencyT<AccountId>,
	OnUnbalanced: OnUnbalancedT<Currency::NegativeImbalance>,
	R: TakeRevenue,
>(
	Weight,
	Currency::Balance,
	PhantomData<(WeightToFee, AssetId, AccountId, Currency, OnUnbalanced, R)>,
);
impl<
		WeightToFee: WeightToFeeT<Balance = Currency::Balance>,
		AssetId: Get<MultiLocation>,
		AccountId,
		Currency: CurrencyT<AccountId>,
		OnUnbalanced: OnUnbalancedT<Currency::NegativeImbalance>,
		R: TakeRevenue,
	> WeightTrader for LocalAssetTrader<WeightToFee, AssetId, AccountId, Currency, OnUnbalanced, R>
{
	fn new() -> Self {
		Self(Weight::zero(), Zero::zero(), PhantomData)
	}

	fn buy_weight(&mut self, weight: XcmWeight, payment: Assets) -> Result<Assets, XcmError> {
		log::trace!(target: "xcm::weight", "LocalAssetTrader::buy_weight weight: {:?}, payment: {:?}", weight, payment);
		let amount = WeightToFee::weight_to_fee(&weight);
		let u128_amount: u128 = amount.try_into().map_err(|_| XcmError::Overflow)?;
		let required: MultiAsset = (Concrete(AssetId::get()), u128_amount).into();
		let unused = payment.checked_sub(required.clone()).map_err(|_| XcmError::TooExpensive)?;
		self.0 = self.0.saturating_add(weight);
		self.1 = self.1.saturating_add(amount);
		R::take_revenue(required);
		Ok(unused)
	}

	fn refund_weight(&mut self, weight: XcmWeight) -> Option<MultiAsset> {
		log::trace!(target: "xcm::weight", "LocalAssetTrader::refund_weight weight: {:?}", weight);
		let weight = weight.min(self.0);
		let amount = WeightToFee::weight_to_fee(&weight);
		self.0 -= weight;
		self.1 = self.1.saturating_sub(amount);
		let amount: u128 = amount.saturated_into();
		if amount > 0 {
			Some((AssetId::get(), amount).into())
		} else {
			None
		}
	}
}
impl<
		WeightToFee: WeightToFeeT<Balance = Currency::Balance>,
		AssetId: Get<MultiLocation>,
		AccountId,
		Currency: CurrencyT<AccountId>,
		OnUnbalanced: OnUnbalancedT<Currency::NegativeImbalance>,
		R: TakeRevenue,
	> Drop for LocalAssetTrader<WeightToFee, AssetId, AccountId, Currency, OnUnbalanced, R>
{
	fn drop(&mut self) {
		OnUnbalanced::on_unbalanced(Currency::issue(self.1));
	}
}

/// We use this adapter to make a limit for non-reserve assets.
/// Refer to https://github.com/paritytech/polkadot/blob/release-v0.9.43/xcm/xcm-builder/src/fungibles_adapter.rs#L312
pub struct LimitFungiblesAdapter<
	Assets,
	Matcher,
	AccountIdConverter,
	AccountId,
	CheckAsset,
	CheckingAccount,
>(PhantomData<(Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount)>);
impl<
		Assets: frame_support::traits::fungibles::Mutate<AccountId>,
		Matcher: xcm_executor::traits::MatchesFungibles<Assets::AssetId, Assets::Balance>,
		AccountIdConverter: Convert<MultiLocation, AccountId>,
		AccountId: Clone, // can't get away without it since Currency is generic over it.
		CheckAsset: xcm_builder::AssetChecking<Assets::AssetId>,
		CheckingAccount: Get<AccountId>,
	> xcm_executor::traits::TransactAsset
	for LimitFungiblesAdapter<Assets, Matcher, AccountIdConverter, AccountId, CheckAsset, CheckingAccount>
{
	fn can_check_in(origin: &MultiLocation, what: &MultiAsset, context: &XcmContext) -> XcmResult {
		xcm_builder::FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::can_check_in(origin, what, context)
	}

	fn check_in(origin: &MultiLocation, what: &MultiAsset, context: &XcmContext) {
		xcm_builder::FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::check_in(origin, what, context)
	}

	fn can_check_out(dest: &MultiLocation, what: &MultiAsset, context: &XcmContext) -> XcmResult {
		xcm_builder::FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::can_check_out(dest, what, context)
	}

	fn check_out(dest: &MultiLocation, what: &MultiAsset, context: &XcmContext) {
		xcm_builder::FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::check_out(dest, what, context)
	}

	fn deposit_asset(what: &MultiAsset, who: &MultiLocation, context: &XcmContext) -> XcmResult {
		xcm_builder::FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::deposit_asset(what, who, context)
	}

	fn withdraw_asset(
		what: &MultiAsset,
		who: &MultiLocation,
		maybe_context: Option<&XcmContext>,
	) -> sp_std::result::Result<xcm_executor::Assets, XcmError> {
		xcm_builder::FungiblesMutateAdapter::<
			Assets,
			Matcher,
			AccountIdConverter,
			AccountId,
			CheckAsset,
			CheckingAccount,
		>::withdraw_asset(what, who, maybe_context)
	}

	fn internal_transfer_asset(
		what: &MultiAsset,
		from: &MultiLocation,
		to: &MultiLocation,
		context: &XcmContext,
	) -> sp_std::result::Result<xcm_executor::Assets, XcmError> {
		xcm_builder::FungiblesTransferAdapter::<Assets, Matcher, AccountIdConverter, AccountId>::internal_transfer_asset(
			what, from, to, context
		)
	}
}
