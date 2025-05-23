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

use std::hash::{Hash, Hasher};
use std::str::FromStr;

use strict_encoding::stl::{Alpha, AlphaNum, AsciiPrintable};
use strict_encoding::RString;
use strict_types::StrictVal;

use crate::LIB_NAME_RGB_CONTRACT;

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT, dumb = Self::from(RString::strict_dumb()))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Ticker(RString<Alpha, AlphaNum, 2, 8>);

impl PartialEq for Ticker {
    fn eq(&self, other: &Self) -> bool {
        self.as_str()
            .to_uppercase()
            .eq(&other.as_str().to_uppercase())
    }
}

impl Hash for Ticker {
    fn hash<H: Hasher>(&self, state: &mut H) { self.as_str().to_uppercase().hash(state) }
}

impl_ident_type!(Ticker);
impl_ident_subtype!(Ticker);

impl Ticker {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self { Self::from_str(&value.unwrap_string()).unwrap() }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct AssetName(RString<AsciiPrintable, AsciiPrintable, 1, 40>);

impl_ident_type!(AssetName);
impl_ident_subtype!(AssetName);

impl AssetName {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self { Self::from_str(&value.unwrap_string()).unwrap() }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, PartialEq, Eq, Hash, Debug, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Details(RString<AsciiPrintable, AsciiPrintable, 1, 0xFF>);
