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
    AssignmentsFilter, ContractIface, ContractOp, IfaceClass, IfaceId, IfaceWrapper, Output,
};
use rgbstd::persistence::ContractStateRead;
use rgbstd::{ContractId, SchemaId, WitnessInfo, XWitnessId};
use strict_encoding::InvalidRString;

use super::{Inflation, PrimaryIssue, Rgb20, Rgb20Info};
use crate::stl::{Amount, AssetSpec, ContractTerms, Details, Precision};
use crate::IssuerWrapper;

pub const RGB20_FIXED_IFACE_ID: IfaceId = IfaceId::from_array([
    0x3a, 0xad, 0xab, 0x95, 0xf5, 0x46, 0x9b, 0xef, 0xab, 0x7a, 0x58, 0xaa, 0xd7, 0x42, 0x71, 0xba,
    0xd1, 0x41, 0x87, 0x32, 0x97, 0x42, 0x6c, 0x6d, 0x7d, 0x8c, 0xe0, 0x4d, 0xdd, 0x6a, 0x55, 0xf4,
]);
pub const RGB20_RENAMABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xf1, 0x57, 0xbd, 0x9e, 0x48, 0xa7, 0x93, 0xea, 0xcd, 0x5d, 0xa8, 0x38, 0x26, 0x36, 0xfa, 0xea,
    0x1b, 0xf0, 0x1c, 0x4c, 0xad, 0xb0, 0x05, 0x2f, 0x0e, 0xce, 0x5b, 0x5e, 0x36, 0x3d, 0x9f, 0xb8,
]);
pub const RGB20_INFLATABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xe2, 0xe8, 0x4c, 0x05, 0xc2, 0x65, 0xea, 0x7c, 0x09, 0xef, 0xd9, 0x6c, 0x54, 0xda, 0x35, 0x13,
    0x60, 0x41, 0xd7, 0xda, 0xdd, 0x96, 0xb7, 0x50, 0x1a, 0xdd, 0x57, 0xdc, 0x8a, 0xaa, 0xdd, 0x80,
]);
pub const RGB20_INFLATABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xd4, 0x9a, 0xe5, 0xab, 0x7a, 0x00, 0x50, 0xc4, 0xae, 0xf3, 0x63, 0x5d, 0x15, 0x11, 0x6a, 0xe9,
    0x3a, 0x6a, 0x9d, 0x7d, 0x1a, 0x17, 0x11, 0xf5, 0xec, 0xc3, 0xcd, 0x63, 0x6a, 0xf5, 0x52, 0xab,
]);
pub const RGB20_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x93, 0xf8, 0x8f, 0xc6, 0xb0, 0xfb, 0xf0, 0xe8, 0x3b, 0x9f, 0x56, 0x76, 0xb9, 0xec, 0xc1, 0x57,
    0xc6, 0x47, 0xf9, 0xd5, 0xe6, 0xde, 0xd4, 0x71, 0x81, 0x58, 0x86, 0xeb, 0x1e, 0x41, 0x56, 0x77,
]);
pub const RGB20_RENAMABLE_INFLATABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x51, 0x96, 0xbd, 0xe8, 0xfa, 0xa5, 0xb4, 0x6d, 0x64, 0x59, 0x15, 0x56, 0x07, 0x9c, 0xb2, 0x56,
    0x34, 0x02, 0xbb, 0x92, 0x38, 0x58, 0xa9, 0x3e, 0x3c, 0x6f, 0xd4, 0x64, 0xc9, 0xe6, 0x1e, 0xf9,
]);
pub const RGB20_REPLACABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xb6, 0xc1, 0x5b, 0xb4, 0xb7, 0x3b, 0x8a, 0x36, 0xc9, 0xd0, 0x2d, 0xd8, 0x74, 0x8d, 0x9b, 0xeb,
    0x06, 0x9e, 0x5b, 0xea, 0x1f, 0x5f, 0xe0, 0xde, 0x71, 0x8e, 0x60, 0x6c, 0x4b, 0x34, 0x79, 0x6e,
]);
pub const RGB20_RENAMABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xe1, 0xd5, 0xb0, 0xf4, 0x86, 0x29, 0xcb, 0x34, 0x1d, 0x16, 0x08, 0x63, 0xc1, 0x5f, 0xa5, 0x88,
    0xa8, 0xf4, 0x2b, 0x78, 0x61, 0xb7, 0x93, 0x76, 0x22, 0x15, 0x8c, 0xab, 0x55, 0x8a, 0xd8, 0xa5,
]);
pub const RGB20_RENAMABLE_INFLATABLE_BURNABLE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xbd, 0xcd, 0xde, 0xce, 0xee, 0x78, 0x47, 0x8e, 0x30, 0x00, 0x92, 0x8a, 0x52, 0x7c, 0x73, 0x5f,
    0x6a, 0x32, 0xdc, 0x3e, 0x0d, 0xdc, 0xaa, 0x9d, 0x53, 0xc9, 0xa1, 0x42, 0x9e, 0x5e, 0xbd, 0x00,
]);
pub const RGB20_FULL_IFACE_ID: IfaceId = IfaceId::from_array([
    0x76, 0x16, 0xd9, 0xef, 0xb2, 0x22, 0x12, 0x35, 0xa7, 0xd2, 0xfa, 0xf8, 0x74, 0x8d, 0x0d, 0x30,
    0xf1, 0xc3, 0x96, 0x99, 0xcd, 0x1d, 0x88, 0x5a, 0xba, 0xba, 0x48, 0x40, 0x31, 0xa6, 0x08, 0x66,
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
        self.0
            .global_typed::<AssetSpec>("spec")
            .expect("RGB20 interface requires global state `spec`")
            .next()
            .expect("RGB20 interface requires global state `spec` to have at least one item")
    }

    pub fn balance(&self, filter: impl AssignmentsFilter) -> Amount {
        self.allocations(filter)
            .map(|output| output.state)
            .sum::<Amount>()
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<Amount>> + 'c {
        self.0
            .outputs_typed("assetOwner", filter)
            .expect("RGB20 interface requires `assetOwner` state")
    }

    pub fn inflation_allowance_allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<Amount>> + 'c {
        self.0
            .outputs_typed("inflationAllowance", filter)
            .expect("RGB20 interface requires `inflationAllowance` state")
    }

    pub fn update_right<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<Amount>> + 'c {
        self.0
            .outputs_typed("updateRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn burn_epoch<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<Amount>> + 'c {
        self.0
            .outputs_typed("burnEpoch", filter)
            .expect("RGB20 interface requires `burnEpoch` state")
    }

    pub fn burn_right<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<Amount>> + 'c {
        self.0
            .outputs_typed("burnRight", filter)
            .expect("RGB20 interface requires `updateRight` state")
    }

    pub fn contract_terms(&self) -> ContractTerms {
        self.0
            .global_typed::<ContractTerms>("terms")
            .expect("RGB20 interface requires global `terms`")
            .next()
            .expect("RGB20 interface requires global state `terms` to have at least one item")
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global_typed::<Amount>("issuedSupply")
            .expect("RGB20 interface requires global `issuedSupply`")
            .sum()
    }

    // Max supply for the inflation asset, if there is no `max supply`, then it will
    // default to the non-inflatable asset `issued supply`
    pub fn max_supply(&self) -> Amount {
        self.0
            .global_typed::<Amount>("maxSupply")
            .unwrap_or_else(|_| {
                self.0
                    .global_typed("issuedSupply")
                    .expect("RGB20 interface requires global `issuedSupply`")
            })
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global_typed::<Amount>("burnedSupply")
            .into_iter()
            .flatten()
            .sum()
    }

    pub fn total_replaced_supply(&self) -> Amount {
        self.0
            .global_typed::<Amount>("replacedSupply")
            .into_iter()
            .flatten()
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
