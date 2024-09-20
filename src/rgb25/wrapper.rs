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
    AssignmentsFilter, ContractIface, FungibleAllocation, IfaceClass, IfaceId, IfaceWrapper,
};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{ContractTerms, Details, Name};
use rgbstd::{AssetTag, ContractId, SchemaId, WitnessInfo, XWitnessId};
use strict_encoding::InvalidRString;

use super::{Issue, Rgb25, Rgb25Info};
use crate::IssuerWrapper;

pub const RGB25_BASE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x05, 0xd2, 0xa2, 0x30, 0x7b, 0x9b, 0x45, 0x94, 0xd8, 0xad, 0xb4, 0xb5, 0xdc, 0x6d, 0xf0, 0xb7,
    0xae, 0x2e, 0x21, 0xc8, 0x72, 0x3c, 0xc4, 0x05, 0xd0, 0xa9, 0xa6, 0xb1, 0x88, 0x1e, 0x32, 0x46,
]);

pub const RGB25_IFACE_ID: IfaceId = IfaceId::from_array([
    0x09, 0x02, 0xdc, 0xc8, 0x58, 0x1c, 0x4c, 0xc0, 0xa1, 0xc5, 0x94, 0x3a, 0xff, 0xc3, 0xb0, 0x77,
    0x31, 0xce, 0xca, 0xda, 0xe6, 0x85, 0x3f, 0x50, 0x0f, 0xcb, 0x4b, 0x78, 0x7c, 0xbc, 0x65, 0x41,
]);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rgb25Wrapper<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> IfaceWrapper<S> for Rgb25Wrapper<S> {
    type Info = Rgb25Info;

    fn with(iface: ContractIface<S>) -> Self {
        if !Rgb25::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB25 interface");
        }
        Self(iface)
    }

    fn info(&self) -> Self::Info { todo!() }

    #[inline]
    fn contract_id(&self) -> ContractId { self.0.contract_id() }

    #[inline]
    fn schema_id(&self) -> SchemaId { self.0.state.schema_id() }

    #[inline]
    fn witness_info(&self, witness_id: XWitnessId) -> Option<WitnessInfo> {
        self.0.witness_info(witness_id)
    }
}

impl<S: ContractStateRead> Rgb25Wrapper<S> {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb25>>(
        issuer: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet::<C>(issuer, name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb25>>(
        issuer: &str,
        name: &str,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet_det::<C>(issuer, name, precision, asset_tag)
    }

    pub fn name(&self) -> Name {
        let strict_val = &self
            .0
            .global("name")
            .expect("RGB25 interface requires global `name`")
            .next()
            .expect("RGB25 interface requires global state `name`");
        Name::from_strict_val_unchecked(strict_val)
    }

    pub fn details(&self) -> Option<Details> {
        self.0
            .global("details")
            .expect("RGB25 interface requires global state `details`")
            .next()
            .map(|strict_val| Details::from_strict_val_unchecked(&strict_val))
    }

    pub fn precision(&self) -> Precision {
        let strict_val = &self
            .0
            .global("precision")
            .expect("RGB25 interface requires global state `precision`")
            .next()
            .expect("RGB25 interface requires global state `precision` to have at least one item");
        Precision::from_strict_val_unchecked(strict_val)
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = FungibleAllocation> + 'c {
        self.0
            .fungible("assetOwner", filter)
            .expect("RGB25 interface requires `assetOwner` state")
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB25 interface requires global state `issuedSupply`")
            .map(|strict_val| Amount::from_strict_val_unchecked(&strict_val))
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .into_iter()
            .flatten()
            .map(|strict_val| Amount::from_strict_val_unchecked(&strict_val))
            .sum()
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB25 interface requires global state `terms`")
            .next()
            .expect("RGB25 interface requires global state `terms` to have at least one item");
        ContractTerms::from_strict_val_unchecked(strict_val)
    }
}
