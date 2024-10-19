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

#![allow(unused_braces)] // caused by rustc unable to understand strict_dumb

use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use amplify::confinement::{Confined, NonEmptyString, SmallOrdSet, SmallString, U8};
use amplify::Bytes32;
use strict_encoding::stl::{Alpha, AlphaNum, AsciiPrintable};
use strict_encoding::{
    InvalidRString, RString, StrictDeserialize, StrictDumb, StrictEncode, StrictSerialize,
    StrictType,
};

use super::{MediaType, Precision, ProofOfReserves, LIB_NAME_RGB_CONTRACT};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct BurnMeta {
    pub burn_proofs: SmallOrdSet<ProofOfReserves>,
}
impl StrictSerialize for BurnMeta {}
impl StrictDeserialize for BurnMeta {}

impl Display for BurnMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "burnProofs {{")?;
        for proof in &self.burn_proofs {
            writeln!(f, "  {proof}")?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct IssueMeta {
    pub reserves: SmallOrdSet<ProofOfReserves>,
}
impl StrictSerialize for IssueMeta {}
impl StrictDeserialize for IssueMeta {}

impl Display for IssueMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "reserves {{")?;
        for proof in &self.reserves {
            writeln!(f, "  {proof}")?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT, dumb = { Article::from("DUMB") })]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Article(RString<Alpha, AlphaNum, 1, 32>);

impl_ident_type!(Article);
impl_ident_subtype!(Article);

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT, dumb = { Ticker::from("DUMB") })]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Ticker(RString<Alpha, AlphaNum, 1, 8>);

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

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display, FromStr)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Name(RString<AsciiPrintable, AsciiPrintable, 1, 40>);

impl StrictSerialize for Name {}
impl StrictDeserialize for Name {}

impl_ident_type!(Name);
impl_ident_subtype!(Name);

#[derive(Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, From)]
#[wrapper(Deref, Display)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct Details(NonEmptyString<U8>);
impl StrictSerialize for Details {}
impl StrictDeserialize for Details {}

impl AsRef<str> for Details {
    #[inline]
    fn as_ref(&self) -> &str { self.0.as_str() }
}

impl StrictDumb for Details {
    fn strict_dumb() -> Self {
        Self(Confined::try_from(s!("Dumb long description which is stupid and so on...")).unwrap())
    }
}

impl FromStr for Details {
    type Err = InvalidRString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Confined::try_from_iter(s.chars())?;
        Ok(Self(s))
    }
}

impl From<&'static str> for Details {
    fn from(s: &'static str) -> Self { Self::from_str(s).expect("invalid details") }
}

impl TryFrom<String> for Details {
    type Error = InvalidRString;

    fn try_from(name: String) -> Result<Self, InvalidRString> {
        let s = Confined::try_from(name)?;
        Ok(Self(s))
    }
}

impl Debug for Details {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ContractDetails")
            .field(&self.as_str())
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct AssetSpec {
    pub ticker: Ticker,
    pub name: Name,
    pub details: Option<Details>,
    pub precision: Precision,
}
impl StrictSerialize for AssetSpec {}
impl StrictDeserialize for AssetSpec {}

impl Display for AssetSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ticker {}, name {}, details {}, precision {}",
            self.ticker,
            self.name,
            self.details
                .as_ref()
                .map(Details::to_string)
                .unwrap_or_else(|| s!("~")),
            self.precision
        )
    }
}

impl AssetSpec {
    pub fn new(ticker: &'static str, name: &'static str, precision: Precision) -> AssetSpec {
        AssetSpec {
            ticker: Ticker::from(ticker),
            name: Name::from(name),
            details: None,
            precision,
        }
    }

    pub fn with(
        ticker: &str,
        name: &str,
        precision: Precision,
        details: Option<&str>,
    ) -> Result<AssetSpec, InvalidRString> {
        Ok(AssetSpec {
            ticker: Ticker::try_from(ticker.to_owned())?,
            name: Name::try_from(name.to_owned())?,
            details: details.map(Details::from_str).transpose()?,
            precision,
        })
    }

    pub fn ticker(&self) -> &str { self.ticker.as_str() }

    pub fn name(&self) -> &str { self.name.as_str() }

    pub fn details(&self) -> Option<&str> { self.details.as_ref().map(|d| d.as_str()) }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct ContractSpec {
    pub article: Option<Article>,
    pub name: Name,
    pub details: Option<Details>,
    pub precision: Precision,
}
impl StrictSerialize for ContractSpec {}
impl StrictDeserialize for ContractSpec {}

impl Display for ContractSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("article ")?;
        if let Some(article) = &self.article {
            write!(f, "{article}")?;
        } else {
            f.write_str("~")?;
        }

        write!(f, ", name {}", self.name)?;

        f.write_str(", details ")?;
        if let Some(details) = &self.details {
            write!(f, "{details}")?;
        } else {
            f.write_str("~")?;
        }

        write!(f, ", precision {}", self.precision)
    }
}

impl ContractSpec {
    pub fn new(name: &'static str, precision: Precision) -> ContractSpec {
        ContractSpec {
            article: None,
            name: Name::from(name),
            details: None,
            precision,
        }
    }

    pub fn with(
        article: &str,
        name: &str,
        precision: Precision,
        details: Option<&str>,
    ) -> Result<ContractSpec, InvalidRString> {
        Ok(ContractSpec {
            article: Some(Article::try_from(article.to_owned())?),
            name: Name::try_from(name.to_owned())?,
            details: details.map(Details::from_str).transpose()?,
            precision,
        })
    }

    pub fn article(&self) -> Option<&str> { self.article.as_ref().map(|a| a.as_str()) }

    pub fn name(&self) -> &str { self.name.as_str() }

    pub fn details(&self) -> Option<&str> { self.details.as_ref().map(|d| d.as_str()) }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Display, Default)]
#[display(inner)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
pub struct RicardianContract(SmallString);
impl StrictSerialize for RicardianContract {}
impl StrictDeserialize for RicardianContract {}

impl AsRef<str> for RicardianContract {
    #[inline]
    fn as_ref(&self) -> &str { self.0.as_str() }
}

impl FromStr for RicardianContract {
    type Err = InvalidRString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = Confined::try_from_iter(s.chars())?;
        Ok(Self(s))
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct Attachment {
    #[strict_type(rename = "type")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: MediaType,
    pub digest: Bytes32,
}
impl StrictSerialize for Attachment {}
impl StrictDeserialize for Attachment {}

impl Display for Attachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "type {}, digest 0x{}", self.ty, self.digest)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(StrictDumb, StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_RGB_CONTRACT)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct ContractTerms {
    pub text: RicardianContract,
    pub media: Option<Attachment>,
}
impl StrictSerialize for ContractTerms {}
impl StrictDeserialize for ContractTerms {}

impl Display for ContractTerms {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "text {}", self.text)?;

        f.write_str(", media ")?;
        if let Some(media) = &self.media {
            write!(f, "{media}")?;
        } else {
            f.write_str("~")?;
        }

        Ok(())
    }
}
