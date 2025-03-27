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

use commit_verify::ReservedBytes;

use crate::LIB_NAME_RGB21;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Display, Default)]
#[display("alignmentBytes")]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Fe256Align8(ReservedBytes<31>);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Display, Default)]
#[display("alignmentBytes")]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Fe256Align16(ReservedBytes<30>);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Display, Default)]
#[display("alignmentBytes")]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Fe256Align32(ReservedBytes<28>);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Display, Default)]
#[display("alignmentBytes")]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Fe256Align64(ReservedBytes<24>);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Display, Default)]
#[display("alignmentBytes")]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Fe256Align128(ReservedBytes<16>);
