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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_types;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

mod traits;

pub mod rgb20;
pub mod rgb21;
pub mod rgb25;

pub use rgb20::{Rgb20, Rgb20Info, Rgb20Wrapper};
pub use rgb21::{Rgb21, Rgb21Wrapper};
pub use rgb25::{Rgb25, Rgb25Info, Rgb25Wrapper};
pub use traits::{IssuerWrapper, SchemaIssuer};

pub const LNPBP_IDENTITY: &str = "ssi:LZS1ux-gjD9nXPF-OcetUUkW-6r3uSCS6-aQhs9W5f-8JE7w";

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display(uppercase)]
pub enum IfaceStandard {
    Rgb20,
    Rgb21,
    Rgb25,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display("unknown RGB interface standard name")]
pub struct UnknownStandard;

mod _from_str {
    use std::str::FromStr;

    use super::*;

    impl FromStr for IfaceStandard {
        type Err = UnknownStandard;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_uppercase().as_str() {
                "RGB20" => Ok(Self::Rgb20),
                "RGB21" => Ok(Self::Rgb21),
                "RGB25" => Ok(Self::Rgb25),
                _ => Err(UnknownStandard),
            }
        }
    }
}
