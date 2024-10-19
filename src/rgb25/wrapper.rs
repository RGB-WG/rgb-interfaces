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
    AssignmentsFilter, ContractIface, IfaceClass, IfaceId, IfaceWrapper, Output,
};
use rgbstd::persistence::ContractStateRead;
use rgbstd::{ContractId, SchemaId, WitnessInfo, XWitnessId};
use strict_encoding::InvalidRString;

use super::{Issue, Rgb25, Rgb25Info};
use crate::stl::{Amount, ContractTerms, Details, Name, Precision};
use crate::IssuerWrapper;

pub const RGB25_BASE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xc4, 0x28, 0x29, 0x06, 0xeb, 0x65, 0x20, 0x6d, 0xfd, 0x36, 0x4f, 0x9a, 0xca, 0x79, 0x80, 0x12,
    0x63, 0x9d, 0xe6, 0xf7, 0x3d, 0xf7, 0x95, 0x86, 0xb5, 0x16, 0xc8, 0x7a, 0x3e, 0x7e, 0x16, 0x26,
]);

pub const RGB25_IFACE_ID: IfaceId = IfaceId::from_array([
    0x35, 0xd3, 0x3e, 0x07, 0xb1, 0xc6, 0xb9, 0xe6, 0x02, 0x13, 0x45, 0x38, 0x5c, 0x41, 0x2e, 0x91,
    0x1b, 0x43, 0x88, 0x67, 0xb3, 0x41, 0x39, 0xf3, 0x62, 0xb6, 0xae, 0x44, 0xd2, 0x52, 0x0a, 0xd7,
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

    pub fn name(&self) -> Name {
        self.0
            .global_typed::<Name>("name")
            .expect("RGB25 interface requires global `name`")
            .next()
            .expect("RGB25 interface requires global state `name`")
    }

    pub fn details(&self) -> Option<Details> {
        self.0
            .global_typed::<Details>("details")
            .expect("RGB25 interface requires global state `details`")
            .next()
    }

    pub fn precision(&self) -> Precision {
        self.0
            .global_typed::<Precision>("precision")
            .expect("RGB25 interface requires global state `precision`")
            .next()
            .expect("RGB25 interface requires global state `precision` to have at least one item")
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<Amount>> + 'c {
        self.0
            .outputs_typed("assetOwner", filter)
            .expect("RGB25 interface requires `assetOwner` state")
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global_typed::<Amount>("issuedSupply")
            .expect("RGB25 interface requires global state `issuedSupply`")
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global_typed::<Amount>("burnedSupply")
            .into_iter()
            .flatten()
            .sum()
    }

    pub fn contract_terms(&self) -> ContractTerms {
        self.0
            .global_typed::<ContractTerms>("terms")
            .expect("RGB25 interface requires global state `terms`")
            .next()
            .expect("RGB25 interface requires global state `terms` to have at least one item")
    }
}
