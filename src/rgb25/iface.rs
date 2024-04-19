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

use rgbstd::interface::{GenesisIface, GlobalIface, Iface, Modifier, VerNo};
use rgbstd::stl::StandardTypes;
use rgbstd::{Identity, Occurrences};

use crate::LNPBP_IDENTITY;

pub fn named_contract() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        name: tn!("NamedContract"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("spec") => GlobalIface::required(types.get("RGBContract.ContractSpec")),
            fname!("terms") => GlobalIface::required(types.get("RGBContract.ContractTerms")),
        },
        assignments: none!(),
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Abstract,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("spec") => Occurrences::Once,
                fname!("terms") => Occurrences::Once,
            },
            assignments: none!(),
            valencies: none!(),
            errors: none!(),
        },
        transitions: none!(),
        extensions: none!(),
        errors: none!(),
        default_operation: None,
    }
}
