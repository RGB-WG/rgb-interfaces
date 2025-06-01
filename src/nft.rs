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

#![allow(unused_braces)]

use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

use amplify::confinement::SmallBlob;
use amplify::Bytes32;
use strict_encoding::stl::AlphaSmall;
use strict_encoding::{
    InvalidRString, RString, RestrictedCharSet, StrictDeserialize, StrictDumb, StrictEncode, StrictSerialize,
};
use strict_types::StrictVal;

use crate::{AssetName, ProofOfReserves, LIB_NAME_RGB21};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
pub struct MediaType {
    #[strict_type(rename = "type")]
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

    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let ty = MediaRegName::from_strict_val_unchecked(value.unwrap_struct("type"));
        let subtype = value
            .unwrap_struct("subtype")
            .unwrap_option()
            .map(MediaRegName::from_strict_val_unchecked);
        let charset = value
            .unwrap_struct("charset")
            .unwrap_option()
            .map(MediaRegName::from_strict_val_unchecked);
        Self { ty, subtype, charset }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.ty, if let Some(subty) = &self.subtype { subty.to_string() } else { s!("*") })
    }
}

impl FromStr for MediaType {
    type Err = ParseMediaTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ty, subty) = s
            .split_once('/')
            .ok_or(ParseMediaTypeError::InvalidStructure)?;
        let ty = MediaRegName::from_str(ty).map_err(ParseMediaTypeError::TypeName)?;
        let subty = if subty == "*" {
            None
        } else {
            Some(MediaRegName::from_str(subty).map_err(ParseMediaTypeError::SubtypeName)?)
        };
        // TODO: Parse charset
        Ok(Self { ty, subtype: subty, charset: None })
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum ParseMediaTypeError {
    /// media type (MIME) must consist of two parts separated by a slash.
    InvalidStructure,
    /// invalid media (MIME) type component; {0}
    TypeName(InvalidRString),
    /// invalid media (MIME) subtype component; {0}
    SubtypeName(InvalidRString),
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21, dumb = { MediaRegName::from("dumb") })]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct MediaRegName(RString<AlphaSmall, MimeChar, 1, 64>);

impl_ident_type!(MediaRegName);
impl_ident_subtype!(MediaRegName);

impl MediaRegName {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        MediaRegName::from_str(&value.unwrap_string()).expect("invalid media reg name")
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21, tags = repr, into_u8, try_from_u8)]
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

#[derive(Wrapper, WrapperMut, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From)]
#[wrapper(Display, FromStr, Add, Sub, Mul, Div, Rem)]
#[wrapper_mut(AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct TokenNo(u32);

#[derive(Wrapper, WrapperMut, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From)]
#[wrapper(Display, FromStr, Add, Sub, Mul, Div, Rem)]
#[wrapper_mut(AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct TokenFractions(u64);

impl TokenFractions {
    pub const ZERO: Self = TokenFractions(0);

    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self { value.unwrap_uint::<u64>().into() }

    pub fn value(self) -> u64 { self.0 }

    pub fn saturating_add(&self, other: impl Into<Self>) -> Self { self.0.saturating_add(other.into().0).into() }
    pub fn saturating_sub(&self, other: impl Into<Self>) -> Self { self.0.saturating_sub(other.into().0).into() }

    pub fn saturating_add_assign(&mut self, other: impl Into<Self>) {
        *self = self.0.saturating_add(other.into().0).into();
    }
    pub fn saturating_sub_assign(&mut self, other: impl Into<Self>) {
        *self = self.0.saturating_sub(other.into().0).into();
    }

    #[must_use]
    pub fn checked_add(&self, other: impl Into<Self>) -> Option<Self> { self.0.checked_add(other.into().0).map(Self) }
    #[must_use]
    pub fn checked_sub(&self, other: impl Into<Self>) -> Option<Self> { self.0.checked_sub(other.into().0).map(Self) }

    #[must_use]
    pub fn checked_add_assign(&mut self, other: impl Into<Self>) -> Option<()> {
        *self = self.0.checked_add(other.into().0).map(Self)?;
        Some(())
    }
    #[must_use]
    pub fn checked_sub_assign(&mut self, other: impl Into<Self>) -> Option<()> {
        *self = self.0.checked_sub(other.into().0).map(Self)?;
        Some(())
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct EmbeddedMedia {
    #[cfg_attr(feature = "serde", serde(with = "serde_with::rust::display_fromstr"))]
    pub mime: MediaType,
    pub data: SmallBlob,
}

impl EmbeddedMedia {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let mime = MediaType::from_strict_val_unchecked(value.unwrap_struct("mime"));
        let data = SmallBlob::from_iter_checked(value.unwrap_struct("data").unwrap_bytes().iter().copied());

        Self { mime, data }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct Attachment {
    #[cfg_attr(feature = "serde", serde(with = "serde_with::rust::display_fromstr"))]
    pub mime: MediaType,
    pub digest: Bytes32,
}
impl StrictSerialize for Attachment {}
impl StrictDeserialize for Attachment {}

impl Attachment {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let mime = MediaType::from_strict_val_unchecked(value.unwrap_struct("mime"));
        let digest = value
            .unwrap_struct("digest")
            .unwrap_bytes()
            .try_into()
            .expect("invalid digest");
        Self { mime, digest }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct Nft {
    pub token_no: TokenNo,
    pub fractions: TokenFractions,
}

impl StrictSerialize for Nft {}
impl StrictDeserialize for Nft {}

impl Nft {
    pub fn new(no: impl Into<TokenNo>, fractions: impl Into<TokenFractions>) -> Self {
        Self { token_no: no.into(), fractions: fractions.into() }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct OwnedNft {
    pub token_no: TokenNo,
    pub fractions: TokenFractions,
}

impl StrictSerialize for OwnedNft {}
impl StrictDeserialize for OwnedNft {}

impl OwnedNft {
    pub fn new(no: impl Into<TokenNo>, fractions: impl Into<TokenFractions>) -> Self {
        Self { token_no: no.into(), fractions: fractions.into() }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Display, Error, From)]
#[display(inner)]
pub enum NftParseError {
    #[display(doc_comments)]
    /// invalid token index {0}.
    InvalidIndex(String),

    #[display(doc_comments)]
    /// invalid fraction {0}.
    InvalidFraction(String),

    #[display(doc_comments)]
    /// allocation must have format <fraction>@<token_index>.
    WrongFormat,
}

impl FromStr for OwnedNft {
    type Err = NftParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('@') {
            return Err(NftParseError::WrongFormat);
        }

        match s.split_once('@') {
            Some((fraction, token_index)) => Ok(OwnedNft {
                token_no: token_index
                    .parse()
                    .map_err(|_| NftParseError::InvalidIndex(token_index.to_owned()))?,
                fractions: fraction
                    .parse()
                    .map_err(|_| NftParseError::InvalidFraction(fraction.to_lowercase()))?,
            }),
            None => Err(NftParseError::WrongFormat),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct NftSpec {
    pub name: Option<AssetName>,
    pub embedded: EmbeddedMedia,
    pub external: Option<Attachment>,
    pub reserves: Option<ProofOfReserves>,
}

impl StrictSerialize for NftSpec {}
impl StrictDeserialize for NftSpec {}

impl NftSpec {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let name = value
            .unwrap_struct("name")
            .unwrap_option()
            .map(|x| AssetName::from_str(&x.unwrap_string()).expect("invalid uda name"));

        let embedded = EmbeddedMedia::from_strict_val_unchecked(value.unwrap_struct("embedded"));

        let external = value
            .unwrap_struct("external")
            .unwrap_option()
            .map(Attachment::from_strict_val_unchecked);

        let reserves = value
            .unwrap_struct("reserves")
            .unwrap_option()
            .map(ProofOfReserves::from_strict_val_unchecked);

        Self { name, embedded, external, reserves }
    }
}

#[cfg(test)]
mod test {
    use strict_types::value::StrictNum;

    use super::*;

    #[test]
    fn owned_fraction_from_str() {
        let owned_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        assert_eq!(owned_fraction.value(), 1);
        assert_eq!(format!("{owned_fraction}"), "1");
    }

    #[test]
    fn owned_fraction_from_strict_val() {
        // note that the strict number is u128 but not u64
        let owned_fraction = TokenFractions::from_strict_val_unchecked(&StrictVal::Number(StrictNum::Uint(1)));

        assert_eq!(owned_fraction.value(), 1);
        assert_eq!(format!("{owned_fraction}"), "1");
    }

    #[test]
    fn owned_fraction_add_assign() {
        let mut owned_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        let _ = owned_fraction.checked_add_assign(TokenFractions::ZERO);
        assert_eq!(owned_fraction.value(), 1);
        assert_eq!(format!("{owned_fraction}"), "1");
    }

    #[test]
    fn owned_fraction_add() {
        let owned_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        let owned = match owned_fraction.checked_add(TokenFractions::ZERO) {
            Some(value) => value,
            None => TokenFractions::ZERO,
        };
        assert_eq!(owned.value(), 1);
        assert_eq!(format!("{owned}"), "1");
    }

    #[test]
    fn owned_fraction_sub() {
        let owned_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        let other_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        let owned = match owned_fraction.checked_sub(other_fraction) {
            Some(value) => value,
            None => TokenFractions::ZERO,
        };
        assert_eq!(owned.value(), 0);
        assert_eq!(format!("{owned}"), "0");
    }

    #[test]
    fn owned_fraction_sub_assign() {
        let mut owned_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        let other_fraction = match TokenFractions::from_str("1") {
            Ok(value) => value,
            Err(_) => TokenFractions::ZERO,
        };

        let _ = owned_fraction.checked_sub_assign(other_fraction);
        assert_eq!(owned_fraction.value(), 0);
        assert_eq!(format!("{owned_fraction}"), "0");
    }
}
