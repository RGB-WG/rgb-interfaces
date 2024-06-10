use std::str::FromStr;

use amplify::confinement::SmallBlob;
use amplify::Wrapper;
use bp::dbc::Method;
use rgbstd::containers::ValidContract;
use rgbstd::interface::{BuilderError, ContractBuilder, IfaceClass, TxOutpoint};
use rgbstd::invoice::{Allocation, Precision};
use rgbstd::stl::{AssetSpec, Attachment, ContractTerms, MediaType, RicardianContract};
use rgbstd::{AltLayer1, AssetTag, GenesisSeal, Identity, TokenIndex};
use strict_encoding::InvalidRString;

use super::{EmbeddedMedia, Rgb21, TokenData};
use crate::{IssuerWrapper, SchemaIssuer};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display(doc_comments)]
pub enum IssuerError {
    /// contract genesis doesn't support allocating to liquid seals; request
    /// liquid support first.
    NoLiquidSupport,
    /// the amount of token fractions in outputs exceeds 1.
    FractionOverflow,
    /// attachment has a type which is not allowed for the token
    InvalidAttachmentType,
    /// allocation of unknown token ID {0}
    UnknownToken,
}

impl From<BuilderError> for IssuerError {
    fn from(err: BuilderError) -> Self {
        match err {
            BuilderError::InvalidLayer1(_) => IssuerError::NoLiquidSupport,
            err => panic!("invalid RGB21 schema. Details: {err}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rgb21PrimaryIssue {
    builder: ContractBuilder,
    terms: ContractTerms,
}

impl Rgb21PrimaryIssue {
    pub fn testnet_with(
        issuer: SchemaIssuer<Rgb21>,
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(issuer, by, ticker, name, details, precision, false)
    }

    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb21>>(
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(C::issuer(), by, ticker, name, details, precision, false)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb21>>(
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Self, InvalidRString> {
        let mut me = Self::testnet_int(C::issuer(), by, ticker, name, details, precision, true)?;
        me.builder = me
            .builder
            .add_asset_tag("assetOwner", asset_tag)
            .expect("invalid RGB20 schema (assetOwner mismatch)");
        Ok(me)
    }
    fn testnet_int(
        issuer: SchemaIssuer<Rgb21>,
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        deterministic: bool,
    ) -> Result<Self, InvalidRString> {
        let spec = AssetSpec::with(ticker, name, precision, details)?;
        let terms = ContractTerms {
            text: RicardianContract::default(),
            media: None,
        };

        let (schema, main_iface_impl, types, scripts, features) = issuer.into_split();
        let mut builder = match deterministic {
            false => ContractBuilder::with(
                Identity::from_str(by).expect("invalid issuer identity string"),
                Rgb21::iface(features),
                schema,
                main_iface_impl,
                types,
                scripts,
            ),
            true => ContractBuilder::deterministic(
                Identity::from_str(by).expect("invalid issuer identity string"),
                Rgb21::iface(features),
                schema,
                main_iface_impl,
                types,
                scripts,
            ),
        };

        builder = builder
            .add_global_state("spec", spec)
            .expect("invalid RGB21 schema (token specification mismatch)");

        Ok(Self { builder, terms })
    }

    pub fn support_liquid(mut self) -> Self {
        self.builder = self
            .builder
            .add_layer1(AltLayer1::Liquid)
            .expect("only one layer1 can be added");
        self
    }

    pub fn add_terms(
        mut self,
        contract: &str,
        media: Option<Attachment>,
    ) -> Result<Self, InvalidRString> {
        let terms = RicardianContract::from_str(contract)?;
        self.terms = ContractTerms { text: terms, media };
        Ok(self)
    }

    pub fn allocate<O: TxOutpoint>(
        mut self,
        method: Method,
        beneficiary: O,
        allocation: impl Into<Allocation>,
    ) -> Result<Self, IssuerError> {
        let allocation = allocation.into();
        let beneficiary = beneficiary.map_to_xchain(|outpoint| {
            GenesisSeal::new_random(method, outpoint.txid, outpoint.vout)
        });
        self.builder = self
            .builder
            .add_data("assetOwner", beneficiary, allocation)?;
        Ok(self)
    }

    pub fn allocate_all<O: TxOutpoint>(
        mut self,
        method: Method,
        allocations: impl IntoIterator<Item = (O, impl Into<Allocation>)>,
    ) -> Result<Self, IssuerError> {
        for (beneficiary, allocation) in allocations {
            self = self.allocate(method, beneficiary, allocation)?;
        }
        Ok(self)
    }

    #[allow(clippy::result_large_err)]
    pub fn issue_contract(
        self,
        token_index: u32,
        image_url: &str,
    ) -> Result<ValidContract, IssuerError> {
        Ok(self
            .pre_issue_contract(token_index, image_url)?
            .issue_contract()?)
    }

    #[allow(clippy::result_large_err)]
    fn pre_issue_contract(
        self,
        token_index: u32,
        image_url: &str,
    ) -> Result<ContractBuilder, IssuerError> {
        let index = TokenIndex::from_inner(token_index);
        let image_bytes = image_url.as_bytes().to_vec();
        let preview = EmbeddedMedia {
            ty: MediaType::with("image/*"),
            data: SmallBlob::try_from_iter(image_bytes).expect("invalid data"),
        };
        let token_data = TokenData {
            index,
            preview: Some(preview),
            ..Default::default()
        };

        Ok(self
            .builder
            .add_global_state("token", token_data)?
            .add_global_state("terms", self.terms)?)
    }
}
