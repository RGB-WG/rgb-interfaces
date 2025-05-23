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

use bc::stl::bp_tx_stl;
use commit_verify::stl::commit_verify_stl;
use strict_types::stl::std_stl;
use strict_types::{LibBuilder, SemId, SymbolicSys, SystemBuilder, TypeLib, TypeSystem};

use crate::{
    Amount, AssetName, AttachmentType, Details, Fe256Align128, Fe256Align16, Fe256Align32, Fe256Align64, Fe256Align8,
    Nft, NftEngraving, NftSpec, Precision, Ticker, LIB_NAME_RGB21, LIB_NAME_RGB_CONTRACT,
};

/// Strict types id for the library providing data types for RGB contracts.
pub const LIB_ID_RGB_INTERFACES: &str = "stl:2aAz0_BS-YAv7Mri-NUPnT31-VUDkbH7-CpQ~aQW-7OZn4n8#stadium-pony-cuba";

/// Strict types id for the library providing data types for RGB21.
pub const LIB_ID_RGB21: &str = "stl:4rgMe2Z3-1sbnfw5-_1DGhwq-hS1yoE4-Ry2Y_TA-0Ir5OI8#slang-size-xray";

pub fn rgb_contract_stl() -> TypeLib {
    LibBuilder::with(libname!(LIB_NAME_RGB_CONTRACT), [std_stl().to_dependency_types()])
        .transpile::<Amount>()
        .transpile::<Precision>()
        .transpile::<Ticker>()
        .transpile::<AssetName>()
        .transpile::<Details>()
        .compile()
        .expect("invalid common types library")
}

pub fn rgb21_stl() -> TypeLib {
    LibBuilder::with(libname!(LIB_NAME_RGB21), [
        std_stl().to_dependency_types(),
        rgb_contract_stl().to_dependency_types(),
        commit_verify_stl().to_dependency_types(),
        bp_tx_stl().to_dependency_types(),
    ])
    .transpile::<Nft>()
    .transpile::<NftSpec>()
    .transpile::<AttachmentType>()
    .transpile::<NftEngraving>()
    .transpile::<Fe256Align8>()
    .transpile::<Fe256Align16>()
    .transpile::<Fe256Align32>()
    .transpile::<Fe256Align64>()
    .transpile::<Fe256Align128>()
    .compile()
    .expect("invalid common types library")
}

#[derive(Debug)]
pub struct CommonTypes(SymbolicSys);

impl Default for CommonTypes {
    fn default() -> Self { CommonTypes::new() }
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

#[derive(Debug)]
pub struct Rgb21Types(SymbolicSys);

impl Default for Rgb21Types {
    fn default() -> Self { Rgb21Types::new() }
}

impl Rgb21Types {
    pub fn new() -> Self {
        Self(
            SystemBuilder::new()
                .import(std_stl())
                .unwrap()
                .import(rgb_contract_stl())
                .unwrap()
                .import(commit_verify_stl())
                .unwrap()
                .import(bp_tx_stl())
                .unwrap()
                .import(rgb21_stl())
                .unwrap()
                .finalize()
                .unwrap(),
        )
    }

    pub fn type_system(&self) -> TypeSystem {
        let types = rgb21_stl().types;
        let types = types.iter().map(|(tn, ty)| ty.sem_id_named(tn));
        self.0.as_types().extract(types).unwrap()
    }

    pub fn get(&self, name: &'static str) -> SemId {
        *self
            .0
            .resolve(name)
            .unwrap_or_else(|| panic!("type '{name}' is absent in RGB21 type library"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn common_lib_id() {
        let lib = rgb_contract_stl();
        assert_eq!(lib.id().to_string(), LIB_ID_RGB_INTERFACES);
    }

    #[test]
    fn rgb21_lib_id() {
        let lib = rgb21_stl();
        assert_eq!(lib.id().to_string(), LIB_ID_RGB21);
    }
}
