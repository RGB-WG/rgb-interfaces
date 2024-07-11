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
use rgbstd::stl::{rgb_contract_stl, AssetSpec, ContractTerms, Details};
use rgbstd::{AssetTag, XWitnessId};
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::iface::*;
use super::{Features, Inflation, PrimaryIssue, Rgb20Info};
use crate::IssuerWrapper;

pub const RGB20_FIXED_IFACE_ID: IfaceId = IfaceId::from_array([
    0xfe, 0x25, 0x27, 0x3b, 0xd6, 0x8e, 0xd7, 0x18, 0x6a, 0x51, 0xde, 0xb5, 0x26, 0x6e, 0x52, 0xe7,
    0xec, 0x0c, 0xde, 0x78, 0x1b, 0xcb, 0x91, 0x95, 0x13, 0x29, 0x50, 0x65, 0x30, 0x0c, 0x60, 0x39,
]);
pub const RGB20_IFACE_ID: IfaceId = IfaceId::from_array([
    0xaf, 0xaf, 0xdd, 0x2d, 0xe5, 0x72, 0x1f, 0x41, 0xd0, 0xa5, 0x4c, 0x85, 0xd9, 0x76, 0xd7, 0xd8,
    0x43, 0xf6, 0x26, 0xa6, 0xc7, 0xa7, 0x0b, 0x08, 0x48, 0x3e, 0x1f, 0xc5, 0x41, 0x1a, 0x38, 0xa4,
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
        let (mut name, mut iface) = if features.renaming {
            (tn!("RGB20Renamable"), rgb20_renamable())
        } else {
            (tn!("RGB20"), rgb20_base())
        };
        if features.inflation.is_fixed() {
            iface = iface.expect_extended(fixed(), tn!(format!("{name}Fixed")));
        } else if features.inflation.is_inflatable() {
            iface = iface.expect_extended(inflatable(), tn!(format!("{name}Inflatable")));
            name = tn!(format!("{name}Inflatable"));
        }
        if features.inflation.is_burnable() {
            iface = iface.expect_extended(burnable(), tn!(format!("{name}Burnable")));
            if features.inflation.is_replaceable() {
                name = tn!(format!("{}Replaceable", name.to_string().replace("Inflatable", "")));
                iface = iface.expect_extended(replaceable(), name);
            }
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

    fn info(&self) -> Self::Info {
        let spec = self.spec();
        let terms = self.contract_terms();
        Rgb20Info {
            contract: self.0.info.clone(),
            ticker: spec.ticker.to_string(),
            name: spec.name.to_string(),
            details: spec.details.as_ref().map(Details::to_string),
            terms: terms.text.to_string(),
            attach: terms.media,
            precision: spec.precision,
            features: self.features(),
            issued: self.total_issued_supply(),
            burned: self.total_burned_supply(),
            replaced: self.total_replaced_supply(),
        }
    }
}

impl Rgb20 {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        issuer: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet::<C>(issuer, ticker, name, details, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
        issuer: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet_det::<C>(issuer, ticker, name, details, precision, asset_tag)
    }

    pub fn features(&self) -> Features {
        let renaming = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "rename");
        let inflatable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "issue");
        let burnable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "burn");
        let replaceable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "replace");

        let inflation = match (inflatable, burnable, replaceable) {
            (true, true, true) => Inflation::Replaceable,
            (true, true, false) => Inflation::InflatableBurnable,
            (false, true, false) => Inflation::Burnable,
            (true, false, false) => Inflation::Inflatable,
            (false, false, false) => Inflation::Fixed,
            (true, false, true) | (false, false, true) => {
                panic!("replaceable asset with no burn enabled")
            }
            (false, true, true) => panic!("replaceable but non-inflatible asset"),
        };

        Features {
            renaming,
            inflation,
        }
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

    // Max supply for the inflation asset, if there is no `max supply`, then it will
    // default to the non-inflatable asset `issued supply`
    pub fn max_supply(&self) -> Amount {
        self.0
            .global("maxSupply")
            .unwrap_or_else(|_| {
                self.0
                    .global("issuedSupply")
                    .expect("RGB20 interface requires global `issuedSupply`")
            })
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
