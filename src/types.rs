// Collection of the standard RGB smart contract interface
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by RGB Consortium members & contributors
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2025 RGB Consortium members & contributors
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use strict_types::stl::std_stl;
use strict_types::{LibBuilder, SemId, SymbolicSys, SystemBuilder, TypeLib, TypeSystem};

use crate::{Amount, AssetName, Precision, Ticker, LIB_NAME_RGB_CONTRACT};

#[derive(Debug)]
pub struct CommonTypes(SymbolicSys);

impl Default for CommonTypes {
    fn default() -> Self { CommonTypes::new() }
}

pub fn rgb_contract_stl() -> TypeLib {
    LibBuilder::new(libname!(LIB_NAME_RGB_CONTRACT), tiny_bset! {
        std_stl().to_dependency(),
    })
    .transpile::<Amount>()
    .transpile::<Precision>()
    .transpile::<Ticker>()
    .transpile::<AssetName>()
    .compile()
    .expect("invalid common types library")
}

impl CommonTypes {
    pub fn new() -> Self {
        Self(
            SystemBuilder::new()
                .import(std_stl())
                .unwrap()
                .import(rgb_contract_stl())
                .unwrap()
                .finalize()
                .unwrap(),
        )
    }

    pub fn type_system(&self) -> TypeSystem {
        let types = rgb_contract_stl().types;
        let types = types.iter().map(|(tn, ty)| ty.sem_id_named(tn));
        self.0.as_types().extract(types).unwrap()
    }

    pub fn get(&self, name: &'static str) -> SemId {
        *self
            .0
            .resolve(name)
            .unwrap_or_else(|| panic!("type '{name}' is absent in RGB contract common type library"))
    }
}
