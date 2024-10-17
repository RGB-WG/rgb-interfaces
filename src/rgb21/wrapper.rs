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
use rgbstd::stl::bp_tx_stl;
use rgbstd::{ContractId, SchemaId, WitnessInfo, XWitnessId};
use strict_types::stl::std_stl;
use strict_types::{CompileError, LibBuilder, TypeLib};

use super::{
    AttachmentType, EngravingData, ItemsCount, NftAllocation, Rgb21, TokenData, LIB_NAME_RGB21,
};
use crate::rgb20::Rgb20Info;
use crate::stl::{rgb_contract_stl, AssetSpec, ContractTerms};

pub const RGB21_UNIQUE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x93, 0x29, 0x82, 0x38, 0x62, 0x1a, 0x36, 0x62, 0xaf, 0xc2, 0x99, 0xf5, 0x77, 0xab, 0x6f, 0x01,
    0x97, 0xb8, 0xbe, 0x29, 0x5f, 0x12, 0xf9, 0xe5, 0xba, 0x03, 0xdf, 0xb3, 0x4c, 0x36, 0x3f, 0xf7,
]);

pub const RGB21_IFACE_ID: IfaceId = IfaceId::from_array([
    0x22, 0xf1, 0x39, 0x64, 0x48, 0x63, 0x2c, 0xfc, 0xe6, 0xb8, 0xa5, 0x2c, 0xb4, 0x55, 0xeb, 0x39,
    0x81, 0x8c, 0x0e, 0x07, 0xe8, 0x3e, 0x2e, 0x47, 0x48, 0xc7, 0xb2, 0x9c, 0x5c, 0xc4, 0x59, 0xfd,
]);

fn _rgb21_stl() -> Result<TypeLib, CompileError> {
    LibBuilder::new(libname!(LIB_NAME_RGB21), tiny_bset! {
        std_stl().to_dependency(),
        bp_tx_stl().to_dependency(),
        rgb_contract_stl().to_dependency(),
    })
    .transpile::<TokenData>()
    .transpile::<EngravingData>()
    .transpile::<ItemsCount>()
    .transpile::<NftAllocation>()
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
        self.0
            .global_typed::<AssetSpec>("spec")
            .expect("RGB21 interface requires global `spec`")
            .next()
            .expect("RGB21 interface requires global state `spec` to have at least one item")
    }

    pub fn contract_terms(&self) -> ContractTerms {
        self.0
            .global_typed::<ContractTerms>("terms")
            .expect("RGB21 interface requires global `terms`")
            .next()
            .expect("RGB21 interface requires global state `terms` to have at least one item")
    }

    pub fn token_data(&self) -> TokenData {
        self.0
            .global_typed::<TokenData>("tokens")
            .expect("RGB21 interface requires global `tokens`")
            .next()
            .expect("RGB21 interface requires global state `tokens` to have at least one item")
    }

    pub fn engraving_data(&self) -> impl Iterator<Item = EngravingData> + '_ {
        self.0
            .global_typed::<EngravingData>("engravings")
            .expect("RGB21 interface requires global state `engravings`")
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl AssignmentsFilter + 'c,
    ) -> impl Iterator<Item = Output<NftAllocation>> + 'c {
        self.0
            .outputs_typed::<NftAllocation>("assetOwner", filter)
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
