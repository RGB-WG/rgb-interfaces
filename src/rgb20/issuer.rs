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

use std::str::FromStr;

use bp::dbc::Method;
use rand::random;
use rgbstd::containers::ValidContract;
use rgbstd::interface::{BuilderError, ContractBuilder, IfaceClass, TxOutpoint};
use rgbstd::{AltLayer1, GenesisSeal, Identity};
use strict_encoding::InvalidRString;

use super::Rgb20;
use crate::stl::{Amount, AssetSpec, Attachment, ContractTerms, Precision, RicardianContract};
use crate::{IssuerWrapper, SchemaIssuer};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display(doc_comments)]
pub enum IssuerError {
    /// contract genesis doesn't support allocating to liquid seals; request
    /// liquid support first.
    NoLiquidSupport,
    /// overflow in the amount of the issued assets: the total amount must not
    /// exceed 2^64.
    AmountOverflow,
}

impl From<BuilderError> for IssuerError {
    fn from(err: BuilderError) -> Self {
        match err {
            BuilderError::InvalidLayer1(_) => IssuerError::NoLiquidSupport,
            err => panic!("invalid RGB20 schema. Details: {err}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrimaryIssue {
    builder: ContractBuilder,
    issued: Amount,
    inflation: Option<Amount>,
    terms: ContractTerms,
}

impl PrimaryIssue {
    pub fn testnet_with(
        issuer: SchemaIssuer<Rgb20>,
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(issuer, by, ticker, name, details, precision)
    }

    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb20>>(
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(C::issuer(), by, ticker, name, details, precision)
    }

    fn testnet_int(
        issuer: SchemaIssuer<Rgb20>,
        by: &str,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        let spec = AssetSpec::with(ticker, name, precision, details)?;
        let terms = ContractTerms {
            text: RicardianContract::default(),
            media: None,
        };

        let (schema, main_iface_impl, types, scripts, features) = issuer.into_split();
        let builder = ContractBuilder::with(
            Identity::from_str(by).expect("invalid issuer identity string"),
            features.iface(),
            schema,
            main_iface_impl,
            types,
            scripts,
        )
        .serialize_global_state("spec", &spec)
        .expect("invalid RGB20 schema (token specification mismatch)");

        Ok(Self {
            builder,
            terms,
            issued: Amount::ZERO,
            inflation: None,
        })
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

    pub fn add_inflation_metadata(mut self, amount: Amount) -> Result<Self, IssuerError> {
        self.builder = self
            .builder
            .serialize_metadata("allowedInflation", &amount)?;
        Ok(self)
    }

    pub fn allocate<O: TxOutpoint>(
        self,
        method: Method,
        beneficiary: O,
        amount: impl Into<Amount>,
    ) -> Result<Self, IssuerError> {
        self.allocate_det(method, beneficiary, random(), amount)
    }

    pub fn allocate_all<O: TxOutpoint>(
        mut self,
        method: Method,
        allocations: impl IntoIterator<Item = (O, impl Into<Amount>)>,
    ) -> Result<Self, IssuerError> {
        for (beneficiary, amount) in allocations {
            self = self.allocate(method, beneficiary, amount)?;
        }
        Ok(self)
    }

    /// Add asset allocation in a deterministic way.
    pub fn allocate_det<O: TxOutpoint>(
        mut self,
        method: Method,
        beneficiary: O,
        seal_blinding: u64,
        amount: impl Into<Amount>,
    ) -> Result<Self, IssuerError> {
        let amount = amount.into();
        let beneficiary = beneficiary.map_to_xchain(|outpoint| {
            GenesisSeal::with_blinding(method, outpoint.txid, outpoint.vout, seal_blinding)
        });
        self.issued
            .checked_add_assign(amount)
            .ok_or(IssuerError::AmountOverflow)?;
        self.builder =
            self.builder
                .serialize_owned_state("assetOwner", beneficiary, &amount, None)?;
        Ok(self)
    }

    pub fn allow_inflation<O: TxOutpoint>(
        self,
        method: Method,
        controller: O,
        supply: impl Into<Amount>,
    ) -> Result<Self, IssuerError> {
        self.allow_inflation_det(method, controller, random(), supply)
    }

    /// Add asset allocation in a deterministic way.
    pub fn allow_inflation_det<O: TxOutpoint>(
        mut self,
        method: Method,
        controller: O,
        seal_blinding: u64,
        supply: impl Into<Amount>,
    ) -> Result<Self, IssuerError> {
        let supply = supply.into();
        let beneficiary = controller.map_to_xchain(|outpoint| {
            GenesisSeal::with_blinding(method, outpoint.txid, outpoint.vout, seal_blinding)
        });
        self = self.update_max_supply(supply)?;
        self.builder =
            self.builder
                .serialize_owned_state("inflationAllowance", beneficiary, &supply, None)?;
        Ok(self)
    }

    fn update_max_supply(mut self, supply: Amount) -> Result<Self, IssuerError> {
        match &mut self.inflation {
            Some(max) => max
                .checked_add_assign(supply)
                .ok_or(IssuerError::AmountOverflow)?,
            None => self.inflation = Some(supply),
        }
        Ok(self)
    }

    pub fn allow_burn<O: TxOutpoint>(
        mut self,
        method: Method,
        controller: O,
    ) -> Result<Self, IssuerError> {
        let controller = controller.map_to_xchain(|outpoint| {
            GenesisSeal::new_random(method, outpoint.txid, outpoint.vout)
        });
        self.builder = self.builder.add_rights("burnRight", controller, None)?;
        Ok(self)
    }

    pub fn allow_burn_det<O: TxOutpoint>(
        mut self,
        method: Method,
        controller: O,
        seal_blinding: u64,
    ) -> Result<Self, IssuerError> {
        let controller = controller.map_to_xchain(|outpoint| {
            GenesisSeal::with_blinding(method, outpoint.txid, outpoint.vout, seal_blinding)
        });
        self.builder = self.builder.add_rights("burnRight", controller, None)?;
        Ok(self)
    }

    pub fn allow_replace<O: TxOutpoint>(
        mut self,
        method: Method,
        controller: O,
    ) -> Result<Self, IssuerError> {
        let controller = controller.map_to_xchain(|outpoint| {
            GenesisSeal::new_random(method, outpoint.txid, outpoint.vout)
        });
        self.builder = self.builder.add_rights("replaceRight", controller, None)?;
        Ok(self)
    }

    pub fn allow_replace_det<O: TxOutpoint>(
        mut self,
        method: Method,
        controller: O,
        seal_blinding: u64,
    ) -> Result<Self, IssuerError> {
        let controller = controller.map_to_xchain(|outpoint| {
            GenesisSeal::with_blinding(method, outpoint.txid, outpoint.vout, seal_blinding)
        });
        self.builder = self.builder.add_rights("replaceRight", controller, None)?;
        Ok(self)
    }

    #[allow(clippy::result_large_err)]
    pub fn issue_contract(self) -> Result<ValidContract, IssuerError> {
        Ok(self.pre_issue_contract()?.issue_contract()?)
    }

    #[allow(clippy::result_large_err)]
    pub fn issue_contract_det(self, timestamp: i64) -> Result<ValidContract, IssuerError> {
        Ok(self.pre_issue_contract()?.issue_contract_det(timestamp)?)
    }

    #[allow(clippy::result_large_err)]
    fn pre_issue_contract(mut self) -> Result<ContractBuilder, IssuerError> {
        if let Some(inflation) = self.inflation {
            let max = self
                .issued
                .checked_add(inflation)
                .ok_or(IssuerError::AmountOverflow)?;
            self.builder = self.builder.serialize_global_state("maxSupply", &max)?;
        }
        Ok(self
            .builder
            .serialize_global_state("issuedSupply", &self.issued)?
            .serialize_global_state("terms", &self.terms)?)
    }
}

// TODO: Add secondary issuer and other actors
