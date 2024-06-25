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
    ContractIface, DataAllocation, Iface, IfaceClass, IfaceId, OutpointFilter, RightsAllocation,
};
use rgbstd::stl::{bp_tx_stl, rgb_contract_stl, AssetSpec, ContractTerms};
use rgbstd::{Allocation, AssetTag, Precision};
use strict_encoding::InvalidRString;
use strict_types::stl::std_stl;
use strict_types::{CompileError, LibBuilder, TypeLib};

use super::iface::*;
use super::issuer::PrimaryIssue;
use super::{
    AttachmentType, EngravingData, Features, Issues, ItemsCount, TokenData, LIB_NAME_RGB21,
};
use crate::rgb20::iface::{named_asset, renameable};
use crate::rgb20::Rgb20Info;
use crate::{IssuerWrapper, SchemaIssuer};

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
fn rgb21_stl() -> TypeLib { _rgb21_stl().expect("invalid strict type RGB21 library") }

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct Rgb21(ContractIface);

impl From<ContractIface> for Rgb21 {
    fn from(iface: ContractIface) -> Self {
        if !Rgb21::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB21 interface");
        }
        Self(iface)
    }
}

impl IfaceClass for Rgb21 {
    const IFACE_NAME: &'static str = LIB_NAME_RGB21;
    const IFACE_IDS: &'static [IfaceId] = &[RGB21_UNIQUE_IFACE_ID, RGB21_IFACE_ID];

    type Features = Features;
    type Info = Rgb20Info;

    fn iface(features: Self::Features) -> Iface {
        let mut iface = named_asset().expect_extended(nft(), "RGB21Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB21Renameable");
        }
        if features.engraving {
            iface = iface.expect_extended(engravable(), "RGB21Engravable");
        }
        iface = match features.issues {
            Issues::Unique => iface.expect_extended(unique(), "RGB21Unique"),
            Issues::Limited => iface.expect_extended(limited(), "RGB21Limited"),
            Issues::MultiIssue => iface.expect_extended(issuable(), "RGB21Issuable"),
        };
        /*
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB21Reservable");
        }
         */
        iface
    }

    fn iface_id(features: Self::Features) -> IfaceId {
        // TODO: Optimize with constants
        Rgb21::iface(features).iface_id()
    }

    fn stl() -> TypeLib { rgb21_stl() }

    fn info(&self) -> Self::Info { todo!() }
}

impl Rgb21 {

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

    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB21 interface requires global `spec`")[0];
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB21 interface requires global `terms`")[0];
        ContractTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn token_data(&self) -> TokenData {
        let strict_val = &self
            .0
            .global("tokens")
            .expect("RGB21 interface requires global `tokens`")[0];
        TokenData::from_strict_val_unchecked(strict_val)
    }

    pub fn engarving_data(&self) -> EngravingData {
        let strict_val = &self
            .0
            .global("engravings")
            .expect("RGB21 interface requires global state `engravings`")[0];
        EngravingData::from_strict_val_unchecked(strict_val)
    }

    pub fn update_right<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = RightsAllocation> + 'c {
        self.0
            .rights("updateRight", filter)
            .expect("RGB21 interface requires `updateRight` state")
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = DataAllocation> + 'c {
        self.0
            .data("assetOwner", filter)
            .expect("RGB21 interface requires `assetOwner` state")
    }

    pub fn features(&self) -> Features {
        let renaming = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "rename");

        let engraving = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "engrave");
        let inflatable = self
            .0
            .iface
            .transitions
            .iter()
            .any(|field| field.name.as_str() == "issue");
        let issues = match inflatable {
            true => Issues::MultiIssue,
            false => Issues::Unique,
            _ => Issues::Limited,
        };

        Features {
            renaming,
            engraving,
            issues,
        }
    }

    pub fn inflation_allowance_allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = DataAllocation> + 'c {
        self.0
            .data("inflationAllowance", filter)
            .expect("RGB21 interface requires `inflationAllowance` state")
    }
}
