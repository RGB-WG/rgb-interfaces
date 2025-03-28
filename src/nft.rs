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

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use amplify::ascii::AsciiString;
use amplify::confinement::{Confined, NonEmptyVec, SmallBlob};
use amplify::{ByteArray, Bytes32};
use bc::{Outpoint, Txid};
use strict_encoding::stl::{AlphaSmall, AsciiPrintable};
use strict_encoding::{
    InvalidRString, RString, RestrictedCharSet, StrictDeserialize, StrictDumb, StrictEncode, StrictSerialize,
    TypedWrite,
};
use strict_types::StrictVal;

use crate::{AssetName, Details, Fe256Align32, Ticker, LIB_NAME_RGB21};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, From)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21, dumb = ProofOfReserves::new(strict_dumb!(), strict_dumb!()))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProofOfReserves {
    pub utxo: Outpoint,
    pub proof: SmallBlob,
}
impl StrictSerialize for ProofOfReserves {}
impl StrictDeserialize for ProofOfReserves {}

impl ProofOfReserves {
    pub fn new(utxo: Outpoint, proof: SmallBlob) -> ProofOfReserves { ProofOfReserves { utxo, proof } }

    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let utxo = value.unwrap_struct("utxo");
        let txid = Txid::from_slice_unsafe(utxo.unwrap_struct("txid").unwrap_bytes());
        let vout: u32 = utxo.unwrap_struct("vout").unwrap_uint();
        let utxo = Outpoint::new(txid, vout);

        let proof = SmallBlob::from_checked(value.unwrap_struct("proof").unwrap_bytes().into());

        Self { utxo, proof }
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub struct TokenIndex(u32);

#[derive(Wrapper, WrapperMut, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From)]
#[wrapper(Display, FromStr, Add, Sub, Mul, Div, Rem)]
#[wrapper_mut(AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct OwnedFraction(u64);

impl OwnedFraction {
    pub const ZERO: Self = OwnedFraction(0);

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
pub struct NftEngraving {
    pub applied_to: TokenIndex,
    pub content: EmbeddedMedia,
}

impl NftEngraving {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let index = TokenIndex::from(
            value
                .unwrap_struct("index")
                .unwrap_num()
                .unwrap_uint::<u32>(),
        );
        let content = EmbeddedMedia::from_strict_val_unchecked(value.unwrap_struct("content"));

        Self { applied_to: index, content }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct EmbeddedMedia {
    #[strict_type(rename = "type")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: MediaType,
    pub data: SmallBlob,
}

impl EmbeddedMedia {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let ty = MediaType::from_strict_val_unchecked(value.unwrap_struct("type"));
        let data = SmallBlob::from_iter_checked(value.unwrap_struct("data").unwrap_bytes().iter().copied());

        Self { ty, data }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21, dumb = { AttachmentType::with(0, "dumb") })]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct AttachmentType {
    pub id: u8,
    pub name: AttachmentName,
}

impl AttachmentType {
    pub fn with(id: u8, name: &'static str) -> AttachmentType {
        AttachmentType { id, name: AttachmentName::from(name) }
    }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display)]
#[derive(StrictType, StrictDumb, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21, dumb = { AttachmentName::from("dumb") })]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct AttachmentName(Confined<AsciiString, 1, 20>);
impl StrictEncode for AttachmentName {
    fn strict_encode<W: TypedWrite>(&self, writer: W) -> std::io::Result<W> {
        let iter = self
            .0
            .as_bytes()
            .iter()
            .map(|c| AsciiPrintable::try_from(*c).unwrap());
        writer.write_newtype::<Self>(&NonEmptyVec::<AsciiPrintable, 20>::try_from_iter(iter).unwrap())
    }
}

// TODO: Ensure all constructors filter invalid characters
impl FromStr for AttachmentName {
    type Err = InvalidRString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = AsciiString::from_ascii(s.as_bytes())?;
        let s = Confined::try_from_iter(s.chars())?;
        Ok(Self(s))
    }
}

impl From<&'static str> for AttachmentName {
    fn from(s: &'static str) -> Self { Self::from_str(s).expect("invalid attachment name") }
}

impl TryFrom<String> for AttachmentName {
    type Error = InvalidRString;

    fn try_from(name: String) -> Result<Self, InvalidRString> {
        let name = AsciiString::from_ascii(name.as_bytes())?;
        let s = Confined::try_from(name)?;
        Ok(Self(s))
    }
}

impl Debug for AttachmentName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AttachmentName")
            .field(&self.as_str())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct Attachment {
    #[strict_type(rename = "type")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: MediaType,
    pub digest: Bytes32,
}
impl StrictSerialize for Attachment {}
impl StrictDeserialize for Attachment {}

impl Attachment {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let ty = MediaType::from_strict_val_unchecked(value.unwrap_struct("type"));
        let digest = value
            .unwrap_struct("digest")
            .unwrap_bytes()
            .try_into()
            .expect("invalid digest");
        Self { ty, digest }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct Nft {
    pub token_index: TokenIndex,
    // We need this to align the data to the size of a field element, so `index` and `amount` get read into different
    // registers
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _align: Fe256Align32,
    pub fraction: OwnedFraction,
}

impl StrictSerialize for Nft {}
impl StrictDeserialize for Nft {}

impl Nft {
    pub fn new(index: impl Into<TokenIndex>, amount: impl Into<OwnedFraction>) -> Self {
        Self {
            token_index: index.into(),
            _align: Fe256Align32::default(),
            fraction: amount.into(),
        }
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

impl FromStr for Nft {
    type Err = NftParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('@') {
            return Err(NftParseError::WrongFormat);
        }

        match s.split_once('@') {
            Some((fraction, token_index)) => Ok(Nft {
                token_index: token_index
                    .parse()
                    .map_err(|_| NftParseError::InvalidIndex(token_index.to_owned()))?,
                _align: Fe256Align32::default(),
                fraction: fraction
                    .parse()
                    .map_err(|_| NftParseError::InvalidFraction(fraction.to_lowercase()))?,
            }),
            None => Err(NftParseError::WrongFormat),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB21)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct NftSpec {
    pub index: TokenIndex,
    pub ticker: Option<Ticker>,
    pub name: Option<AssetName>,
    pub details: Option<Details>,
    pub preview: Option<EmbeddedMedia>,
    pub media: Option<Attachment>,
    pub attachments: Confined<BTreeMap<u8, Attachment>, 0, 20>,
    pub reserves: Option<ProofOfReserves>,
}

impl StrictSerialize for NftSpec {}
impl StrictDeserialize for NftSpec {}

impl NftSpec {
    pub fn from_strict_val_unchecked(value: &StrictVal) -> Self {
        let index = TokenIndex::from(
            value
                .unwrap_struct("index")
                .unwrap_num()
                .unwrap_uint::<u32>(),
        );
        let ticker = value
            .unwrap_struct("ticker")
            .unwrap_option()
            .map(|x| Ticker::from_str(&x.unwrap_string()).expect("invalid uda ticker"));

        let name = value
            .unwrap_struct("name")
            .unwrap_option()
            .map(|x| AssetName::from_str(&x.unwrap_string()).expect("invalid uda name"));

        let details = value
            .unwrap_struct("details")
            .unwrap_option()
            .map(|x| Details::from_str(&x.unwrap_string()).expect("invalid uda details"));

        let preview = value
            .unwrap_struct("preview")
            .unwrap_option()
            .map(EmbeddedMedia::from_strict_val_unchecked);
        let media = value
            .unwrap_struct("media")
            .unwrap_option()
            .map(Attachment::from_strict_val_unchecked);

        let attachments = if let StrictVal::Map(list) = value.unwrap_struct("attachments") {
            Confined::from_iter_checked(
                list.iter()
                    .map(|(k, v)| (k.unwrap_uint(), Attachment::from_strict_val_unchecked(v))),
            )
        } else {
            Confined::default()
        };

        let reserves = value
            .unwrap_struct("reserves")
            .unwrap_option()
            .map(ProofOfReserves::from_strict_val_unchecked);
        Self {
            index,
            ticker,
            name,
            details,
            preview,
            media,
            attachments,
            reserves,
        }
    }
}

#[cfg(test)]
mod test {
    use strict_types::value::StrictNum;

    use super::*;

    #[test]
    fn owned_fraction_from_str() {
        let owned_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        assert_eq!(owned_fraction.value(), 1);
        assert_eq!(format!("{owned_fraction}"), "1");
    }

    #[test]
    fn owned_fraction_from_strict_val() {
        // note that the strict number is u128 but not u64
        let owned_fraction = OwnedFraction::from_strict_val_unchecked(&StrictVal::Number(StrictNum::Uint(1)));

        assert_eq!(owned_fraction.value(), 1);
        assert_eq!(format!("{owned_fraction}"), "1");
    }

    #[test]
    fn owned_fraction_add_assign() {
        let mut owned_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        let _ = owned_fraction.checked_add_assign(OwnedFraction::ZERO);
        assert_eq!(owned_fraction.value(), 1);
        assert_eq!(format!("{owned_fraction}"), "1");
    }

    #[test]
    fn owned_fraction_add() {
        let owned_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        let owned = match owned_fraction.checked_add(OwnedFraction::ZERO) {
            Some(value) => value,
            None => OwnedFraction::ZERO,
        };
        assert_eq!(owned.value(), 1);
        assert_eq!(format!("{owned}"), "1");
    }

    #[test]
    fn owned_fraction_sub() {
        let owned_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        let other_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        let owned = match owned_fraction.checked_sub(other_fraction) {
            Some(value) => value,
            None => OwnedFraction::ZERO,
        };
        assert_eq!(owned.value(), 0);
        assert_eq!(format!("{owned}"), "0");
    }

    #[test]
    fn owned_fraction_sub_assign() {
        let mut owned_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        let other_fraction = match OwnedFraction::from_str("1") {
            Ok(value) => value,
            Err(_) => OwnedFraction::ZERO,
        };

        let _ = owned_fraction.checked_sub_assign(other_fraction);
        assert_eq!(owned_fraction.value(), 0);
        assert_eq!(format!("{owned_fraction}"), "0");
    }
}
