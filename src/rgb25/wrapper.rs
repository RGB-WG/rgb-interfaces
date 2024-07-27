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
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::{rgb_contract_stl, ContractTerms, Details, Name};
use rgbstd::AssetTag;
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::{Features, Issue, Rgb25Info};
use crate::rgb20::iface::*;
use crate::rgb25::iface::named_contract;
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
pub struct Rgb25<S: ContractStateRead>(ContractIface<S>);

impl<S: ContractStateRead> From<ContractIface<S>> for Rgb25<S> {
    fn from(iface: ContractIface<S>) -> Self {
        if !Rgb25::<S>::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB25 interface");
        }
        Self(iface)
    }
}

impl<S: ContractStateRead> IfaceClass for Rgb25<S> {
    const IFACE_NAME: &'static str = "RGB25";
    const IFACE_IDS: &'static [IfaceId] = &[RGB25_BASE_IFACE_ID, RGB25_IFACE_ID];

    type Features = Features;
    type Info = Rgb25Info;

    fn iface(features: Features) -> Iface {
        let mut iface = named_contract().expect_extended(fungible(), "RGB25Base");
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
        Rgb25::<S>::iface(features).iface_id()
    }

    fn stl() -> TypeLib { rgb_contract_stl() }

    fn info(&self) -> Self::Info { todo!() }
}

impl<S: ContractStateRead> Rgb25<S> {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        issuer: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet::<C, S>(issuer, name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
        issuer: &str,
        name: &str,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet_det::<C, S>(issuer, name, precision, asset_tag)
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
