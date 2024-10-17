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

#![allow(unused_braces)]

use std::fmt::{self, Debug};
use std::str::FromStr;

use strict_encoding::stl::AlphaSmall;
use strict_encoding::{
    RString, RestrictedCharSet, StrictDeserialize, StrictDumb, StrictEncode, StrictSerialize,
};

use super::LIB_NAME_RGB_CONTRACT;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "serde_crate"))]
pub struct MediaType {
    #[strict_type(rename = "type")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: MediaRegName,
    pub subtype: Option<MediaRegName>,
    pub charset: Option<MediaRegName>,
}
impl StrictDumb for MediaType {
    fn strict_dumb() -> Self { MediaType::with("text/plain") }
}
impl StrictSerialize for MediaType {}
impl StrictDeserialize for MediaType {}

impl MediaType {
    /// # Safety
    ///
    /// Panics is the provided string is an invalid type specifier.
    pub fn with(s: &'static str) -> Self {
        let (ty, subty) = s.split_once('/').expect("invalid static media type string");
        MediaType {
            ty: MediaRegName::from(ty),
            subtype: if subty == "*" { None } else { Some(MediaRegName::from(subty)) },
            charset: None,
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            self.ty,
            if let Some(subty) = &self.subtype { subty.to_string() } else { s!("*") }
        )
    }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT, dumb = { MediaRegName::from("dumb") })]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct MediaRegName(RString<AlphaSmall, MimeChar, 1, 64>);

impl_ident_type!(MediaRegName);
impl_ident_subtype!(MediaRegName);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT, tags = repr, into_u8, try_from_u8)]
#[display(inner)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum MimeChar {
    #[display("!")]
    Excl = b'!',
    #[display("#")]
    Hash = b'#',
    #[display("$")]
    Dollar = b'$',
    #[display("&")]
    Amp = b'&',
    #[display("+")]
    Plus = b'+',
    #[display("-")]
    Dash = b'-',
    #[display(".")]
    Dot = b'.',
    #[display("0")]
    Zero = b'0',
    #[display("1")]
    One = b'1',
    #[display("2")]
    Two = b'2',
    #[display("3")]
    Three = b'3',
    #[display("4")]
    Four = b'4',
    #[display("5")]
    Five = b'5',
    #[display("6")]
    Six = b'6',
    #[display("7")]
    Seven = b'7',
    #[display("8")]
    Eight = b'8',
    #[display("9")]
    Nine = b'9',
    #[display("^")]
    Caret = b'^',
    #[display("_")]
    Lodash = b'_',
    #[strict_type(dumb)]
    #[display("a")]
    a = b'a',
    #[display("b")]
    b = b'b',
    #[display("c")]
    c = b'c',
    #[display("d")]
    d = b'd',
    #[display("e")]
    e = b'e',
    #[display("f")]
    f = b'f',
    #[display("g")]
    g = b'g',
    #[display("h")]
    h = b'h',
    #[display("i")]
    i = b'i',
    #[display("j")]
    j = b'j',
    #[display("k")]
    k = b'k',
    #[display("l")]
    l = b'l',
    #[display("m")]
    m = b'm',
    #[display("n")]
    n = b'n',
    #[display("o")]
    o = b'o',
    #[display("p")]
    p = b'p',
    #[display("q")]
    q = b'q',
    #[display("r")]
    r = b'r',
    #[display("s")]
    s = b's',
    #[display("t")]
    t = b't',
    #[display("u")]
    u = b'u',
    #[display("v")]
    v = b'v',
    #[display("w")]
    w = b'w',
    #[display("x")]
    x = b'x',
    #[display("y")]
    y = b'y',
    #[display("z")]
    z = b'z',
}

impl RestrictedCharSet for MimeChar {}
