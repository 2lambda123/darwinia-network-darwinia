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

pragma solidity >=0.8.3;

/// @dev The Staking precompile address
address constant STAKING_ADDRESS = 0x0000000000000000000000000000000000000601;

/// @dev The Staking contract instance
Staking constant STAKING_CONTRACT = Staking(STAKING_ADDRESS);

/// @title Staking
/// @notice The interface of darwinia staking pallet
interface Staking {
	/// @notice Add stakes to the staking pool.
	/// @param ringAmount The amount of staking RING asset
	/// @param ktonAmount The amount of staking KTON asset
	/// @param depositIds The deposit ids list
	/// @return returns true on success, false otherwise.
	function stake(
		uint256 ringAmount,
		uint256 ktonAmount,
		uint8[] memory depositIds
	) external returns (bool);

	/// @notice Withdraw stakes to the staking pool.
	/// @param ringAmount The amount of staking RING asset
	/// @param ktonAmount The amount of staking KTON asset
	/// @param depositIds The deposit ids list
	/// @return returns true on success, false otherwise.
	function unstake(
		uint256 ringAmount,
		uint256 ktonAmount,
		uint8[] memory depositIds
	) external returns (bool);

    /// @notice Re-stake the unstaking assets immediately.
	/// @param ringAmount The amount of staking RING asset
	/// @param ktonAmount The amount of staking KTON asset
	/// @param depositIds The deposit ids list
	/// @return true on success, false otherwise.
	function restake(
		uint256 ringAmount,
		uint256 ktonAmount,
		uint8[] memory depositIds
	) external returns (bool);

    /// @notice Claim the stakes from the pallet/contract account.
	/// @return returns true on success, false otherwise.
	function claim() external returns (bool);

    /// @notice Declare the desire to collect.
    /// @param commission collator commission, 0 ~ 100
	/// @return returns true on success, false otherwise.
	function collect(uint32 commission) external returns (bool);

    /// @notice Declare the desire to nominate a collator.
    /// @param target The target collator address
	/// @return returns true on success, false otherwise.
	function nominate(address target) external returns (bool);

    /// @notice Declare no desire to either collect or nominate.
	/// @return returns true on success, false otherwise.
	function chill() external returns (bool);
}
