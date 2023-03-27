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

// darwinia
use crate::*;
use darwinia_deposit::SimpleAsset;
use dc_types::UNIT;
// substrate
use frame_benchmarking::v2::{self, Linear};
use frame_system::RawOrigin;
use sp_std::prelude::*;

#[v2::benchmarks]
mod benchmarks {
	// darwinia
	use super::*;

	fn deposit_for<T>(who: &T::AccountId, count: u32) -> Vec<DepositId<T>>
	where
		T: Config + darwinia_deposit::Config,
	{
		(0..count.min(<<T as Config>::MaxDeposits>::get()) as u8)
			.map(|x| {
				<darwinia_deposit::Pallet<T>>::lock(
					RawOrigin::Signed(who.to_owned()).into(),
					UNIT,
					1,
				)
				.unwrap();

				x.into()
			})
			.collect()
	}

	#[benchmark]
	fn stake(x: Linear<0, 255>) {
		let a = frame_benchmarking::whitelisted_caller();

		T::RingCurrency::make_free_balance_be(&a, 256 * UNIT);
		<T as darwinia_deposit::Config>::Kton::mint(&a, UNIT).unwrap();

		let deposits = deposit_for::<T>(&a, x);

		// Worst-case scenario:
		//
		// The total number of deposit items has reached `Config::MaxDeposits`.
		#[extrinsic_call]
		_(RawOrigin::Signed(a), UNIT, UNIT, deposits);
	}

	#[benchmark]
	fn unstake(x: Linear<0, 255>) {
		let a = frame_benchmarking::whitelisted_caller();

		T::RingCurrency::make_free_balance_be(&a, 256 * UNIT);
		<T as darwinia_deposit::Config>::Kton::mint(&a, UNIT).unwrap();

		let deposits = deposit_for::<T>(&a, x);

		<Pallet<T>>::stake(RawOrigin::Signed(a.clone()).into(), UNIT, UNIT, deposits.clone())
			.unwrap();

		// Worst-case scenario:
		//
		// The total number of deposit items has reached `Config::MaxDeposits`.
		#[extrinsic_call]
		_(RawOrigin::Signed(a), UNIT, UNIT, deposits);
	}

	#[benchmark]
	fn restake(x: Linear<0, 255>) {
		let a = frame_benchmarking::whitelisted_caller();

		T::RingCurrency::make_free_balance_be(&a, 256 * UNIT);
		<T as darwinia_deposit::Config>::Kton::mint(&a, UNIT).unwrap();

		let deposits = deposit_for::<T>(&a, x);

		<Pallet<T>>::stake(RawOrigin::Signed(a.clone()).into(), UNIT, UNIT, deposits.clone())
			.unwrap();
		<Pallet<T>>::unstake(RawOrigin::Signed(a.clone()).into(), UNIT, UNIT, deposits.clone())
			.unwrap();

		// Worst-case scenario:
		//
		// The total number of deposit items has reached `Config::MaxDeposits`.
		#[extrinsic_call]
		_(RawOrigin::Signed(a), UNIT, UNIT, deposits);
	}

	#[benchmark]
	fn claim() {
		let a = frame_benchmarking::whitelisted_caller();

		T::RingCurrency::make_free_balance_be(&a, 256 * UNIT);
		<T as darwinia_deposit::Config>::Kton::mint(&a, UNIT).unwrap();

		let deposits = deposit_for::<T>(&a, <T as Config>::MaxDeposits::get());

		<Pallet<T>>::stake(RawOrigin::Signed(a.clone()).into(), UNIT, UNIT, deposits.clone())
			.unwrap();
		<Pallet<T>>::unstake(RawOrigin::Signed(a.clone()).into(), UNIT, UNIT, deposits).unwrap();

		<frame_system::Pallet<T>>::set_block_number(
			<frame_system::Pallet<T>>::block_number() + T::MinStakingDuration::get(),
		);

		assert!(<Ledgers<T>>::contains_key(&a));

		// Worst-case scenario:
		//
		// The total number of deposit items has reached `Config::MaxDeposits`.
		// In addition, all `RING` and `KTON` should be claimed, to make ledger get killed.
		#[extrinsic_call]
		_(RawOrigin::Signed(a.clone()));

		assert!(!<Ledgers<T>>::contains_key(&a));
	}

	#[benchmark]
	fn collect() {
		let a = frame_benchmarking::whitelisted_caller();

		// Worst-case scenario:
		//
		// None.
		#[extrinsic_call]
		_(RawOrigin::Signed(a), Default::default());
	}

	#[benchmark]
	fn nominate() {
		let a = frame_benchmarking::whitelisted_caller::<T::AccountId>();
		let a_cloned = a.clone();

		// TODO: https://github.com/paritytech/substrate/pull/13655
		T::RingCurrency::make_free_balance_be(&a, UNIT + 1);

		<Pallet<T>>::stake(
			RawOrigin::Signed(a.clone()).into(),
			UNIT,
			Default::default(),
			Default::default(),
		)
		.unwrap();
		<Pallet<T>>::collect(RawOrigin::Signed(a.clone()).into(), Default::default()).unwrap();

		// Worst-case scenario:
		//
		// Nominate the target collator successfully.
		#[extrinsic_call]
		_(RawOrigin::Signed(a), a_cloned);
	}

	#[benchmark]
	fn chill() {
		let a = frame_benchmarking::whitelisted_caller::<T::AccountId>();

		// TODO: https://github.com/paritytech/substrate/pull/13655
		T::RingCurrency::make_free_balance_be(&a, UNIT + 1);

		<Pallet<T>>::stake(
			RawOrigin::Signed(a.clone()).into(),
			UNIT,
			Default::default(),
			Default::default(),
		)
		.unwrap();
		<Pallet<T>>::collect(RawOrigin::Signed(a.clone()).into(), Default::default()).unwrap();
		<Pallet<T>>::nominate(RawOrigin::Signed(a.clone()).into(), a.clone()).unwrap();

		// Worst-case scenario:
		//
		// Collect and nominate at the same time.
		#[extrinsic_call]
		_(RawOrigin::Signed(a));
	}

	#[benchmark]
	fn set_collator_count() {
		// Worst-case scenario:
		//
		// Set collator count successfully.
		#[extrinsic_call]
		_(RawOrigin::Root, 1);
	}

	frame_benchmarking::impl_benchmark_test_suite!(
		Pallet,
		crate::mock::ExtBuilder::default().build(),
		crate::mock::Runtime
	);
}
