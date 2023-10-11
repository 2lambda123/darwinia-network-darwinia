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

pub use frame_support::traits::{EitherOf, EitherOfDiverse};

// darwinia
use dc_primitives::AccountId;
// frontier
use fp_account::AccountId20;
// substrate
use frame_system::EnsureRoot;
use pallet_collective::{EnsureProportionAtLeast, EnsureProportionMoreThan};

pub type Root = EnsureRoot<AccountId>;
pub type RootOr<T> = EitherOf<Root, T>;
pub type RootOrDiverse<T> = EitherOfDiverse<Root, T>;

pub type RootOrAtLeastHalf<T> = EitherOfDiverse<Root, AtLeastHalf<T>>;
pub type AtLeastHalf<T> = EnsureProportionAtLeast<AccountId, T, 1, 2>;

pub type RootOrMoreThanHalf<T> = EitherOfDiverse<Root, MoreThanHalf<T>>;
pub type MoreThanHalf<T> = EnsureProportionMoreThan<AccountId, T, 1, 2>;

pub type RootOrAtLeastTwoThird<T> = EitherOfDiverse<Root, AtLeastTwoThird<T>>;
pub type AtLeastTwoThird<T> = EnsureProportionAtLeast<AccountId, T, 2, 3>;

pub type RootOrAtLeastThreeFifth<T> = EitherOfDiverse<Root, AtLeastThreeFifth<T>>;
pub type AtLeastThreeFifth<T> = EnsureProportionAtLeast<AccountId, T, 3, 5>;

pub type RootOrAll<T> = EitherOfDiverse<Root, All<T>>;
pub type All<T> = EnsureProportionAtLeast<AccountId, T, 1, 1>;

/// An [`AccountId20`] generated from b"root".
pub const ROOT: AccountId20 =
	AccountId20([0x72, 0x6f, 0x6f, 0x74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
