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

use std::fmt::{self, Display, Formatter, Write};

use chrono::{DateTime, Utc};
use rgbstd::info::ContractInfo;
use rgbstd::stl::Attachment;
use rgbstd::{Amount, Precision, XOutpoint, XWitnessId};

use crate::rgb20::Rgb20;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct Rgb20Info {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub contract: ContractInfo,

    pub ticker: String,
    pub name: String,
    pub details: Option<String>,
    pub terms: String,
    pub attach: Option<Attachment>,
    pub precision: Precision,
    pub features: Rgb20,

    pub issued: Amount,   // TODO: Replace with SupplyInfo
    pub burned: Amount,   // TODO: Replace with SupplyInfo
    pub replaced: Amount, // TODO: Replace with SupplyInfo
}

impl Display for Rgb20Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{: >8}", self.ticker)?;
        f.write_char(f.fill())?;
        write!(f, "{: <40}", self.name)?;
        f.write_char(f.fill())?;
        write!(f, "{: >2}", self.precision as u8)?;
        f.write_char(f.fill())?;
        write!(f, "{}", self.issued)?;
        f.write_char(f.fill())?;
        writeln!(f, "{}", self.features)?;
        if let Some(details) = &self.details {
            writeln!(f, "\t{details}")?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct SupplyInfo {
    pub known: Amount,
    pub max: Amount,
    pub finalized: bool,
    pub disclosed: bool,
    pub events: Vec<SupplyEvent>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct SupplyEvent {
    pub amount: Amount,
    pub timestamp: DateTime<Utc>,
    pub seal: XOutpoint,
    pub witness: XWitnessId,
}
