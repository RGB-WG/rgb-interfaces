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

use rgbstd::interface::{ContractIface, Iface, IfaceClass, IfaceId};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::stl::{rgb_contract_stl, ContractTerms, Details, Name};
use rgbstd::AssetTag;
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::{Features, Issue, Rgb25Info};
use crate::rgb20::iface::*;
use crate::rgb25::iface::named_contract;
use crate::IssuerWrapper;

pub const RGB25_BASE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x5f, 0x06, 0x9e, 0xaa, 0x1b, 0xe8, 0x04, 0x56, 0xad, 0x49, 0x86, 0x27, 0x03, 0x79, 0x2b, 0xdd,
    0x0c, 0x16, 0xdd, 0xf1, 0xb1, 0x36, 0xf9, 0xf9, 0x1f, 0xc4, 0x19, 0xfc, 0xd9, 0x32, 0xe8, 0x2d,
]);

pub const RGB25_IFACE_ID: IfaceId = IfaceId::from_array([
    0x03, 0xd8, 0x1f, 0x64, 0x7e, 0x5e, 0x2c, 0xbd, 0x25, 0xbb, 0xbf, 0x61, 0x0d, 0xf2, 0xb9, 0xef,
    0x66, 0x57, 0xdc, 0xc2, 0xed, 0x6a, 0x5f, 0xc1, 0x1c, 0x2f, 0x4e, 0xd4, 0xbe, 0x35, 0xd2, 0xf6,
]);

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct Rgb25(ContractIface);

impl From<ContractIface> for Rgb25 {
    fn from(iface: ContractIface) -> Self {
        if !Rgb25::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB25 interface");
        }
        Self(iface)
    }
}

impl IfaceClass for Rgb25 {
    const IFACE_NAME: &'static str = "RGB25";
    const IFACE_IDS: &'static [IfaceId] = &[RGB25_BASE_IFACE_ID, RGB25_IFACE_ID];

    type Features = Features;
    type Info = Rgb25Info;

    fn iface(features: Features) -> Iface {
        let mut iface = named_contract().expect_extended(fungible(), "RGB25Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB25Renameable");
        }
        /*
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB25Reservable");
        }
         */
        if features.burnable {
            iface = iface.expect_extended(burnable(), "RGB25Burnable");
        }
        iface
    }

    fn iface_id(features: Self::Features) -> IfaceId {
        // TODO: Optimize with constants
        Rgb25::iface(features).iface_id()
    }

    fn stl() -> TypeLib { rgb_contract_stl() }

    fn info(&self) -> Self::Info { todo!() }
}

impl Rgb25 {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        issuer: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet::<C>(issuer, name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
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
            .expect("RGB25 interface requires global `name`")[0];
        Name::from_strict_val_unchecked(strict_val)
    }

    pub fn details(&self) -> Option<Details> {
        let strict_val = &self
            .0
            .global("details")
            .expect("RGB25 interface requires global `details`");
        if strict_val.len() == 0 {
            None
        } else {
            Some(Details::from_strict_val_unchecked(&strict_val[0]))
        }
    }

    pub fn precision(&self) -> Precision {
        let strict_val = &self
            .0
            .global("precision")
            .expect("RGB25 interface requires global `precision`")[0];
        Precision::from_strict_val_unchecked(strict_val)
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB25 interface requires global `issuedSupply`")
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

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB25 interface requires global `terms`")[0];
        ContractTerms::from_strict_val_unchecked(strict_val)
    }
}
