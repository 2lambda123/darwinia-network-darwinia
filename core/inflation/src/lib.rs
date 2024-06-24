// This file is part of Darwinia.
//
// Copyright (C) Darwinia Network
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

//! Darwinia economic inflation mechanism implementation.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
// #![deny(unused_crate_dependencies)]

#[cfg(test)]
mod test;

// crates.io
use primitive_types::U256;
// darwinia
use dc_types::{Balance, Moment};
// github
use substrate_fixed::types::U94F34;

/// Milliseconds per year.
pub const MILLISECS_PER_YEAR: Balance = (366 * 24 * 60 * 60) * 1000;

/// Issuing maps for ages 1 to 100 years.
pub const ISSUING_MAP: [Balance; 100] = [
	80000000000000000000000000,
	111773286000000000000000000,
	134746987000000000000000000,
	152702244000000000000000000,
	167131169000000000000000000,
	178823309000000000000000000,
	188269287000000000000000000,
	195807995000000000000000000,
	201691935000000000000000000,
	206120192000000000000000000,
	209256584000000000000000000,
	211240393000000000000000000,
	212192983000000000000000000,
	212222105000000000000000000,
	211424760000000000000000000,
	209889162000000000000000000,
	207696140000000000000000000,
	204920129000000000000000000,
	201629917000000000000000000,
	197889213000000000000000000,
	193757076000000000000000000,
	189288266000000000000000000,
	184533528000000000000000000,
	179539841000000000000000000,
	174350616000000000000000000,
	169005897000000000000000000,
	163542519000000000000000000,
	157994277000000000000000000,
	152392074000000000000000000,
	146764063000000000000000000,
	141135790000000000000000000,
	135530328000000000000000000,
	129968413000000000000000000,
	124468570000000000000000000,
	119047241000000000000000000,
	113718915000000000000000000,
	108496242000000000000000000,
	103390155000000000000000000,
	98409990000000000000000000,
	93563591000000000000000000,
	88857424000000000000000000,
	84296682000000000000000000,
	79885385000000000000000000,
	75626479000000000000000000,
	71521926000000000000000000,
	67572800000000000000000000,
	63779363000000000000000000,
	60141155000000000000000000,
	56657064000000000000000000,
	53325400000000000000000000,
	50143962000000000000000000,
	47110103000000000000000000,
	44220789000000000000000000,
	41472652000000000000000000,
	38862045000000000000000000,
	36385086000000000000000000,
	34037705000000000000000000,
	31815679000000000000000000,
	29714676000000000000000000,
	27730281000000000000000000,
	25858032000000000000000000,
	24093442000000000000000000,
	22432030000000000000000000,
	20869335000000000000000000,
	19400942000000000000000000,
	18022495000000000000000000,
	16729714000000000000000000,
	15518406000000000000000000,
	14384477000000000000000000,
	13323941000000000000000000,
	12332926000000000000000000,
	11407684000000000000000000,
	10544591000000000000000000,
	9740153000000000000000000,
	8991010000000000000000000,
	8293934000000000000000000,
	7645831000000000000000000,
	7043743000000000000000000,
	6484844000000000000000000,
	5966438000000000000000000,
	5485963000000000000000000,
	5040980000000000000000000,
	4629177000000000000000000,
	4248363000000000000000000,
	3896462000000000000000000,
	3571515000000000000000000,
	3271673000000000000000000,
	2995191000000000000000000,
	2740429000000000000000000,
	2505842000000000000000000,
	2289982000000000000000000,
	2091489000000000000000000,
	1909087000000000000000000,
	1741584000000000000000000,
	1587865000000000000000000,
	1446887000000000000000000,
	1317679000000000000000000,
	1199333000000000000000000,
	1091005000000000000000000,
	991910000000000000000000,
];

/// Calculate the issuing of a period.
///
/// Use `U94F34` here, because `2^94 > TOTAL_SUPPLY * 10^18`.
pub fn issuing_in_period(period: Moment, elapsed: Moment) -> Option<Balance> {
	let years = (elapsed / MILLISECS_PER_YEAR) as usize;
	let to_issue = ISSUING_MAP[years];
	let to_issue_per_millisecs = U94F34::checked_from_num(to_issue)? / MILLISECS_PER_YEAR;

	Some(to_issue_per_millisecs.checked_mul(U94F34::checked_from_num(period)?)?.floor().to_num())
}

/// Calculate the reward of a deposit.
///
/// Reference(s):
/// - <https://github.com/evolutionlandorg/bank/blob/master/contracts/GringottsBank.sol#L280>
pub fn deposit_interest(amount: Balance, months: u8) -> Balance {
	// The result of `((quot - 1) * precision + rem * precision / d)` is `197` when months is
	// `12`.
	//
	// The default interest is `1_000`.
	// So, we directly use `1_970_000` here instead `interest * 197 * 10^7`.
	fn f(amount: U256, precision: U256, quot: U256, rem: U256, d: U256) -> Option<Balance> {
		Some(
			(amount.checked_mul(
				precision.checked_mul(quot.checked_sub(1_u8.into())?)? + precision * rem / d,
			)? / 1_970_000_u32)
				.as_u128(),
		)
	}

	let amount = U256::from(amount);
	let months = U256::from(months);
	let n = U256::from(67_u8).pow(months);
	let d = U256::from(66_u8).pow(months);
	let quot = n / d;
	let rem = n % d;
	let precision = U256::from(1_000_u16);

	f(amount, precision, quot, rem, d).unwrap_or_default()
}
