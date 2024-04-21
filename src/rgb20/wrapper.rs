// RGB interfaces by LNP/BP Standards Association
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use rgbstd::interface::{
    AmountChange, ContractIface, FungibleAllocation, Iface, IfaceClass, IfaceId, IfaceOp,
    OutpointFilter, RightsAllocation, WitnessFilter,
};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::stl::{rgb_contract_stl, AssetSpec, ContractTerms};
use rgbstd::{AssetTag, XWitnessId};
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::iface::*;
use super::{Features, PrimaryIssue, Rgb20Info};
use crate::IssuerWrapper;

pub const RGB20_FIXED_IFACE_ID: IfaceId = IfaceId::from_array([
    0x82, 0xd9, 0x3e, 0x0c, 0xf5, 0x0a, 0x17, 0xb8, 0xae, 0xa8, 0xdd, 0x83, 0x0b, 0x16, 0x92, 0x07,
    0x19, 0x94, 0xbb, 0x1c, 0x08, 0x64, 0x2c, 0x75, 0xe2, 0xd8, 0xfd, 0x5d, 0x41, 0xc3, 0xca, 0xeb,
]);
pub const RGB20_IFACE_ID: IfaceId = IfaceId::from_array([
    0xba, 0x64, 0xdd, 0x70, 0x90, 0x49, 0x58, 0x52, 0xb1, 0x1a, 0x8e, 0xfe, 0x35, 0x94, 0xd6, 0xc8,
    0x31, 0x99, 0x05, 0xbd, 0xc9, 0x9d, 0x9d, 0x33, 0x87, 0xbe, 0xa7, 0x30, 0x21, 0x9d, 0x5d, 0x03,
]);

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug, From)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct Rgb20(ContractIface);

impl IfaceClass for Rgb20 {
    const IFACE_NAME: &'static str = "RGB20";
    const IFACE_IDS: &'static [IfaceId] = &[RGB20_FIXED_IFACE_ID, RGB20_IFACE_ID];

    type Features = Features;
    type Info = Rgb20Info;

    fn iface(features: Features) -> Iface {
        let mut iface = named_asset().expect_extended(fungible(), "RGB20Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB20Renamable");
        }
        if features.inflation.is_fixed() {
            iface = iface.expect_extended(fixed(), "RGB20Fixed");
        } else if features.inflation.is_inflatable() {
            iface = iface.expect_extended(inflatable(), "RGB20Inflatable");
        }
        if features.inflation.is_burnable() {
            iface = iface.expect_extended(burnable(), "RGB20Burnable");
        }
        if features.inflation.is_replaceable() {
            iface = iface.expect_extended(replaceable(), "RGB20Replaceable");
        }
        /* TODO: Complete reservable interface
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB20Reservable");
        }
         */
        iface
    }

    fn iface_id(features: Self::Features) -> IfaceId {
        // TODO: Optimize with constants
        Rgb20::iface(features).iface_id()
    }

    fn stl() -> TypeLib { rgb_contract_stl() }

    fn info(&self) -> Self::Info { todo!() }
}

impl Rgb20 {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet::<C>(ticker, name, details, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet_det::<C>(ticker, name, details, precision, asset_tag)
    }

    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB20 interface requires global state `spec`")[0];
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn balance(&self, filter: impl OutpointFilter) -> Amount {
        self.allocations(filter)
            .map(|alloc| alloc.state)
            .sum::<Amount>()
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("assetOwner", filter)
            .expect("RGB20 interface requires `assetOwner` state")
    }

    pub fn inflation_allowance_allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("inflationAllowance", filter)
            .expect("RGB20 interface requires `inflationAllowance` state")
    }

    pub fn update_right<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("updateRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn burn_epoch<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnEpoch", filter)
            .expect("RGB20 interface requires `burnEpoch` state")
    }

    pub fn burn_right<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB20 interface requires global `terms`")[0];
        ContractTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB20 interface requires global `issuedSupply`")
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .unwrap_or_default()
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_replaced_supply(&self) -> Amount {
        self.0
            .global("replacedSupply")
            .unwrap_or_default()
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_supply(&self) -> Amount { self.total_issued_supply() - self.total_burned_supply() }

    pub fn transfer_history(
        &self,
        witness_filter: impl WitnessFilter + Copy,
        outpoint_filter: impl OutpointFilter + Copy,
    ) -> HashMap<XWitnessId, IfaceOp<AmountChange>> {
        self.0
            .fungible_ops("assetOwner", witness_filter, outpoint_filter)
            .expect("state name is not correct")
    }
}
