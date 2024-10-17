// RGB standard library for working with smart contracts on Bitcoin & Lightning
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2019-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association. All rights reserved.
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

mod specs;
#[allow(clippy::module_inception)]
mod stl;
mod mime;
mod chain;
mod asset;

pub use asset::{Amount, Precision};
pub use chain::ProofOfReserves;
pub use mime::{MediaRegName, MediaType};
pub use specs::{
    Article, AssetSpec, Attachment, BurnMeta, ContractSpec, ContractTerms, Details, IssueMeta,
    Name, RicardianContract, Ticker,
};
pub use stl::{rgb_contract_stl, StandardTypes, LIB_ID_RGB_CONTRACT};

pub const LIB_NAME_RGB_CONTRACT: &str = "RGBContract";
