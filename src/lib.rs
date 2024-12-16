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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod fungible;
mod types;
mod names;

pub use fungible::*;
pub use names::{AssetName, Ticker};
pub use types::{rgb_contract_stl, CommonTypes};

pub const LIB_NAME_RGB_CONTRACT: &str = "RGBContract";
