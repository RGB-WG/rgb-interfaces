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
use rgbstd::stl::{rgb_contract_stl, AssetTerms, Details, Name};
use rgbstd::AssetTag;
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::{Features, Issue, Rgb25Info};
use crate::rgb20::iface::*;
use crate::IssuerWrapper;

pub const RGB25_BASE_IFACE_ID: IfaceId = IfaceId::from_array([
    0xec, 0x62, 0x9d, 0x0d, 0x0e, 0x21, 0x6c, 0x76, 0x1d, 0x3c, 0x4f, 0x33, 0x86, 0x58, 0x09, 0x8c,
    0xdd, 0x6f, 0x8f, 0x58, 0x2d, 0x79, 0x11, 0x24, 0x59, 0x71, 0x2c, 0x48, 0xb3, 0xb8, 0xb5, 0x8b,
]);

pub const RGB25_IFACE_ID: IfaceId = IfaceId::from_array([
    0x97, 0x9a, 0xf4, 0x76, 0xaa, 0xc3, 0x46, 0xfc, 0xd7, 0x10, 0xc6, 0x04, 0x95, 0x35, 0x2b, 0x29,
    0x87, 0x37, 0xbb, 0x61, 0x4d, 0x31, 0xb4, 0x9f, 0xfd, 0x8f, 0xf6, 0xac, 0x9e, 0x02, 0x0d, 0x99,
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
        let mut iface = named_asset().expect_extended(fungible(), "RGB25Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB25Renameable");
        }
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB25Reservable");
        }
        if features.burnable {
            iface = iface.expect_extended(burnable(), "RGB25Burnable");
        }
        if features == Features::ALL {
            iface.name = Self::IFACE_NAME.into();
        }
        iface
    }

    fn iface_id(features: Self::Features) -> IfaceId { todo!() }

    fn stl() -> TypeLib { rgb_contract_stl() }

    fn info(&self) -> Self::Info { todo!() }
}

impl Rgb25 {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet::<C>(name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
        name: &str,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet_det::<C>(name, precision, asset_tag)
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

    pub fn contract_terms(&self) -> AssetTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB25 interface requires global `terms`")[0];
        AssetTerms::from_strict_val_unchecked(strict_val)
    }
}
