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
    AssignmentsFilter, ContractIface, ContractOp, DataAllocation, IfaceClass, IfaceId, IfaceWrapper,
};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{bp_tx_stl, rgb_contract_stl, AssetSpec, ContractTerms};
use rgbstd::{Allocation, ContractId, SchemaId, WitnessInfo, XWitnessId};
use strict_types::stl::std_stl;
use strict_types::{CompileError, LibBuilder, TypeLib};

use super::{AttachmentType, EngravingData, ItemsCount, Rgb21, TokenData, LIB_NAME_RGB21};
use crate::rgb20::Rgb20Info;

pub const RGB21_UNIQUE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xcd, 0xa8, 0x94, 0x87, 0x6e, 0xc5, 0xd9, 0xc6, 0x16, 0x7d, 0xc7, 0x45, 0x7c, 0xbe, 0x65, 0x05,
    0xcb, 0x94, 0x7a, 0x73, 0xba, 0xe8, 0x86, 0x19, 0x13, 0x40, 0xfd, 0x19, 0xe5, 0x48, 0xbc, 0x65,
]);

pub const RGB21_IFACE_ID: IfaceId = IfaceId::from_array([
    0x31, 0x36, 0xc2, 0xd3, 0x12, 0x32, 0xb7, 0x89, 0x23, 0x9d, 0x13, 0xba, 0x96, 0xb7, 0x9f, 0x31,
    0x34, 0x03, 0x0f, 0x1b, 0x52, 0x35, 0x23, 0x4e, 0x1d, 0xe9, 0xff, 0x58, 0x47, 0xb2, 0xc9, 0xf7,
]);

fn _rgb21_stl() -> Result<TypeLib, CompileError> {
    LibBuilder::new(libname!(LIB_NAME_RGB21), tiny_bset! {
        std_stl().to_dependency(),
        bp_tx_stl().to_dependency(),
        rgb_contract_stl().to_dependency()
    })
    .transpile::<TokenData>()
    .transpile::<EngravingData>()
    .transpile::<ItemsCount>()
    .transpile::<Allocation>()
    .transpile::<AttachmentType>()
    .compile()
}

/// Generates strict type library providing data types for RGB21 interface.
pub fn rgb21_stl() -> TypeLib { _rgb21_stl().expect("invalid strict type RGB21 library") }

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rgb21Wrapper<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> IfaceWrapper<S> for Rgb21Wrapper<S> {
    type Info = Rgb20Info;

    fn with(iface: ContractIface<S>) -> Self {
        if !Rgb21::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB21 interface");
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

impl<S: ContractStateRead> Rgb21Wrapper<S> {
    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB21 interface requires global `spec`")
            .next()
            .expect("RGB21 interface requires global state `spec` to have at least one item");
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB21 interface requires global `terms`")
            .next()
            .expect("RGB21 interface requires global state `terms` to have at least one item");
        ContractTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn token_data(&self) -> TokenData {
        let strict_val = &self
            .0
            .global("tokens")
            .expect("RGB21 interface requires global `tokens`")
            .next()
            .expect("RGB21 interface requires global state `tokens` to have at least one item");
        TokenData::from_strict_val_unchecked(strict_val)
    }

    pub fn engraving_data(&self) -> impl Iterator<Item = EngravingData> + '_ {
        self.0
            .global("engravings")
            .expect("RGB21 interface requires global state `engravings`")
            .map(|strict_val| EngravingData::from_strict_val_unchecked(&strict_val))
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = DataAllocation> + 'c {
        self.0
            .data("assetOwner", filter)
            .expect("RGB21 interface requires `assetOwner` state")
    }

    pub fn history(
        &self,
        filter_outpoints: impl AssignmentsFilter + Clone,
        filter_witnesses: impl AssignmentsFilter + Clone,
    ) -> Vec<ContractOp> {
        self.0.history(filter_outpoints, filter_witnesses)
    }
}
