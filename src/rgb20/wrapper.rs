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

use rgbstd::interface::{
    AssignmentsFilter, ContractIface, ContractOp, FungibleAllocation, IfaceClass, IfaceId,
    IfaceWrapper, RightsAllocation,
};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{AssetSpec, ContractTerms, Details};
use rgbstd::{AssetTag, ContractId, SchemaId, WitnessInfo, XWitnessId};
use strict_encoding::InvalidRString;

use super::{Inflation, PrimaryIssue, Rgb20, Rgb20Info};
use crate::IssuerWrapper;

pub const RGB20_FIXED_IFACE_ID: IfaceId = IfaceId::from_array([
    0xfe, 0x25, 0x27, 0x3b, 0xd6, 0x8e, 0xd7, 0x18, 0x6a, 0x51, 0xde, 0xb5, 0x26, 0x6e, 0x52, 0xe7,
    0xec, 0x0c, 0xde, 0x78, 0x1b, 0xcb, 0x91, 0x95, 0x13, 0x29, 0x50, 0x65, 0x30, 0x0c, 0x60, 0x39,
]);
pub const RGB20_RENAMABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x99, 0xe1, 0xeb, 0xb5, 0x54, 0xd9, 0xeb, 0x10, 0x1d, 0x77, 0x4c, 0x2b, 0x8c, 0x3b, 0x6d, 0x2e,
    0x17, 0x2c, 0xda, 0x7e, 0xf3, 0x9e, 0xc6, 0x41, 0xcd, 0xfc, 0x33, 0x15, 0xc5, 0x3e, 0x8b, 0x6e,
]);
pub const RGB20_INFLATABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x60, 0xba, 0x66, 0x6e, 0x76, 0x36, 0xc9, 0x99, 0xff, 0x25, 0xd1, 0xa0, 0x86, 0x3c, 0x28, 0x08,
    0xfc, 0xe3, 0xdb, 0x5e, 0x72, 0xd3, 0xf5, 0xee, 0xc0, 0x0f, 0x74, 0xa3, 0xf0, 0x8f, 0xfe, 0x98,
]);
pub const RGB20_INFLATABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x05, 0x6c, 0xb6, 0x1f, 0x43, 0x5d, 0xca, 0x48, 0x1a, 0x9d, 0xda, 0xe6, 0x47, 0x97, 0xf2, 0x76,
    0x25, 0x3f, 0xee, 0xd1, 0x54, 0xd9, 0x1f, 0xb3, 0xd4, 0x5f, 0x41, 0x31, 0x95, 0xb4, 0xb4, 0xaa,
]);
pub const RGB20_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xcb, 0x14, 0x11, 0x70, 0xd2, 0x78, 0xc8, 0xc9, 0xc9, 0xd3, 0xf7, 0x20, 0x2e, 0x3c, 0x29, 0x75,
    0x2f, 0x03, 0xb8, 0x98, 0xc5, 0x9e, 0x95, 0x90, 0xe2, 0x2e, 0x42, 0x4e, 0x8d, 0x91, 0xfd, 0x4e,
]);
pub const RGB20_RENAMABLE_INFLATABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x1b, 0x37, 0x77, 0x0f, 0x05, 0x72, 0x4e, 0xd1, 0x77, 0x67, 0xa5, 0x61, 0xf7, 0xcd, 0x6a, 0x8d,
    0x45, 0x95, 0xf5, 0x09, 0x9b, 0x4b, 0xf4, 0x81, 0xe5, 0x3c, 0xa0, 0xeb, 0x2e, 0x4f, 0x2c, 0xf7,
]);
pub const RGB20_REPLACABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xc3, 0x46, 0x75, 0x65, 0xad, 0xa3, 0x92, 0x5a, 0x5f, 0x5b, 0x24, 0xf8, 0x72, 0x83, 0x81, 0xfb,
    0xaf, 0x74, 0xa9, 0xa9, 0x16, 0xe4, 0x07, 0xe3, 0x98, 0x87, 0x3d, 0x7a, 0x36, 0x5b, 0x29, 0x95,
]);
pub const RGB20_RENAMABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x14, 0x57, 0xcf, 0x3d, 0x60, 0xc7, 0xe3, 0xf0, 0x56, 0x06, 0x7e, 0xf7, 0x42, 0x71, 0x3c, 0x59,
    0x59, 0xc7, 0x8f, 0xdf, 0x04, 0xf0, 0x8a, 0x00, 0x21, 0x9b, 0xed, 0xe2, 0x48, 0x97, 0xed, 0x80,
]);
pub const RGB20_RENAMABLE_INFLATABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x32, 0x44, 0xae, 0xbe, 0xfc, 0xf6, 0x4a, 0xd1, 0xb3, 0xb9, 0x5f, 0x07, 0x7b, 0x0a, 0xb4, 0x7e,
    0x88, 0x4c, 0x8a, 0x05, 0xa6, 0xfd, 0x43, 0x31, 0x5e, 0xd7, 0x4f, 0x44, 0x29, 0xd9, 0xc9, 0xe7,
]);
pub const RGB20_FULL_IFACE_ID: IfaceId = IfaceId::from_array([
    0xaf, 0xaf, 0xdd, 0x2d, 0xe5, 0x72, 0x1f, 0x41, 0xd0, 0xa5, 0x4c, 0x85, 0xd9, 0x76, 0xd7, 0xd8,
    0x43, 0xf6, 0x26, 0xa6, 0xc7, 0xa7, 0x0b, 0x08, 0x48, 0x3e, 0x1f, 0xc5, 0x41, 0x1a, 0x38, 0xa4,
]);

#[derive(Clone, Eq, PartialEq, Debug, From)]
pub struct Rgb20Wrapper<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> IfaceWrapper<S> for Rgb20Wrapper<S> {
    type Info = Rgb20Info;

    fn with(iface: ContractIface<S>) -> Self {
        if !Rgb20::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB20 interface");
        }
        Self(iface)
    }

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

    #[inline]
    fn contract_id(&self) -> ContractId { self.0.contract_id() }

    #[inline]
    fn schema_id(&self) -> SchemaId { self.0.state.schema_id() }

    #[inline]
    fn witness_info(&self, witness_id: XWitnessId) -> Option<WitnessInfo> {
        self.0.witness_info(witness_id)
    }
}

impl<S: ContractStateRead> Rgb20Wrapper<S> {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb20>>(
        issuer: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet::<C>(issuer, ticker, name, details, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb20>>(
        issuer: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<PrimaryIssue, InvalidRString> {
        PrimaryIssue::testnet_det::<C>(issuer, ticker, name, details, precision, asset_tag)
    }

    pub fn features(&self) -> Rgb20 {
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

        Rgb20 {
            renaming,
            inflation,
        }
    }

    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB20 interface requires global state `spec`")
            .next()
            .expect("RGB20 interface requires global state `spec` to have at least one item");
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn balance(&self, filter: impl AssignmentsFilter) -> Amount {
        self.allocations(filter)
            .map(|alloc| alloc.state)
            .sum::<Amount>()
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("assetOwner", filter)
            .expect("RGB20 interface requires `assetOwner` state")
    }

    pub fn inflation_allowance_allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("inflationAllowance", filter)
            .expect("RGB20 interface requires `inflationAllowance` state")
    }

    pub fn update_right<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("updateRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn burn_epoch<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnEpoch", filter)
            .expect("RGB20 interface requires `burnEpoch` state")
    }

    pub fn burn_right<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("burnRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB20 interface requires global `terms`")
            .next()
            .expect("RGB20 interface requires global state `terms` to have at least one item");
        ContractTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB20 interface requires global `issuedSupply`")
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
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
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .into_iter()
            .flatten()
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    pub fn total_replaced_supply(&self) -> Amount {
        self.0
            .global("replacedSupply")
            .into_iter()
            .flatten()
            .map(|amount| Amount::from_strict_val_unchecked(&amount))
            .sum()
    }

    pub fn total_supply(&self) -> Amount { self.total_issued_supply() - self.total_burned_supply() }

    pub fn history(
        &self,
        filter_outpoints: impl AssignmentsFilter + Clone,
        filter_witnesses: impl AssignmentsFilter + Clone,
    ) -> Vec<ContractOp> {
        self.0.history(filter_outpoints, filter_witnesses)
    }
}
