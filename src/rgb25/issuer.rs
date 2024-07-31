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
use rgbstd::containers::ValidContract;
use rgbstd::interface::{BuilderError, ContractBuilder, IfaceClass, TxOutpoint};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::persistence::PersistedState;
use rgbstd::stl::{Attachment, ContractTerms, Details, Name, RicardianContract};
use rgbstd::{AltLayer1, AssetTag, BlindingFactor, GenesisSeal, Identity};
use strict_encoding::InvalidRString;

use super::Rgb25;
use crate::rgb20::IssuerError;
use crate::{IssuerWrapper, SchemaIssuer};

#[derive(Clone, Debug)]
pub struct Issue {
    builder: ContractBuilder,
    issued: Amount,
    terms: ContractTerms,
    deterministic: bool,
}

impl Issue {
    fn testnet_int(
        issuer: SchemaIssuer<Rgb25>,
        by: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
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
        .add_global_state("name", Name::try_from(name.to_owned())?)
        .expect("invalid RGB25 schema (name mismatch)")
        .add_global_state("precision", precision)
        .expect("invalid RGB25 schema (precision mismatch)");

        Ok(Self {
            builder,
            terms,
            issued: Amount::ZERO,
            deterministic: false,
        })
    }

    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb25>>(
        by: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(C::issuer(), by, name, precision)
    }

    pub fn testnet_with(
        issuer: SchemaIssuer<Rgb25>,
        by: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(issuer, by, name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb25>>(
        by: &str,
        name: &str,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Self, InvalidRString> {
        let mut me = Self::testnet_int(C::issuer(), by, name, precision)?;
        me.builder = me
            .builder
            .add_asset_tag("assetOwner", asset_tag)
            .expect("invalid RGB25 schema (assetOwner mismatch)");
        me.deterministic = true;
        Ok(me)
    }

    pub fn support_liquid(mut self) -> Self {
        self.builder = self
            .builder
            .add_layer1(AltLayer1::Liquid)
            .expect("only one layer1 can be added");
        self
    }

    pub fn add_details(mut self, details: &str) -> Result<Self, InvalidRString> {
        self.builder = self
            .builder
            .add_global_state("details", Details::try_from(details.to_owned())?)
            .expect("invalid RGB25 schema (details mismatch)");
        Ok(self)
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
        amount: Amount,
    ) -> Result<Self, IssuerError> {
        debug_assert!(
            !self.deterministic,
            "for creating deterministic contracts please use allocate_det method"
        );

        let beneficiary = beneficiary.map_to_xchain(|outpoint| {
            GenesisSeal::new_random(method, outpoint.txid, outpoint.vout)
        });
        self.issued
            .checked_add_assign(amount)
            .ok_or(IssuerError::AmountOverflow)?;
        self.builder =
            self.builder
                .add_fungible_state("assetOwner", beneficiary, amount.value())?;
        Ok(self)
    }

    pub fn allocate_all<O: TxOutpoint>(
        mut self,
        method: Method,
        allocations: impl IntoIterator<Item = (O, Amount)>,
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
        amount: Amount,
        amount_blinding: BlindingFactor,
    ) -> Result<Self, IssuerError> {
        debug_assert!(
            self.deterministic,
            "to add asset allocation in deterministic way the contract builder has to be created \
             using `*_det` constructor"
        );

        let tag = self
            .builder
            .asset_tag("assetOwner")
            .expect("internal library error: asset tag is unassigned");
        let beneficiary = beneficiary.map_to_xchain(|outpoint| {
            GenesisSeal::with_blinding(method, outpoint.txid, outpoint.vout, seal_blinding)
        });
        self.issued
            .checked_add_assign(amount)
            .ok_or(IssuerError::AmountOverflow)?;
        self.builder = self.builder.add_owned_state_det(
            "assetOwner",
            beneficiary,
            PersistedState::Amount(amount, amount_blinding, tag),
        )?;
        Ok(self)
    }

    // TODO: implement when bulletproofs are supported
    /*
    pub fn conceal_allocations(mut self) -> Self {

    }
     */

    #[allow(clippy::result_large_err)]
    pub fn issue_contract(self) -> Result<ValidContract, BuilderError> {
        self.pre_issue_contract().issue_contract()
    }

    #[allow(clippy::result_large_err)]
    pub fn issue_contract_det(self, timestamp: i64) -> Result<ValidContract, BuilderError> {
        self.pre_issue_contract().issue_contract_det(timestamp)
    }

    #[allow(clippy::result_large_err)]
    fn pre_issue_contract(self) -> ContractBuilder {
        self.builder
            .add_global_state("issuedSupply", self.issued)
            .expect("invalid RGB25 schema (issued supply mismatch)")
            .add_global_state("terms", self.terms)
            .expect("invalid RGB25 schema (contract terms mismatch)")
    }

    // TODO: Add secondary issuance and other methods
}
