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

use bp::seals::txout::CloseMethod;
use bp::Outpoint;
use rgbstd::containers::ValidContract;
use rgbstd::interface::{BuilderError, ContractBuilder, IfaceClass};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::persistence::PersistedState;
use rgbstd::stl::{Attachment, ContractTerms, Details, Name, RicardianContract};
use rgbstd::{GenesisSeal, Identity};
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
        close_method: CloseMethod,
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
            close_method,
            Identity::from_str(by).expect("invalid issuer identity string"),
            features.iface(),
            schema,
            main_iface_impl,
            types,
            scripts,
            rgbstd::Layer1::Bitcoin,
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
        close_method: CloseMethod,
        by: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(close_method, C::issuer(), by, name, precision)
    }

    pub fn testnet_with(
        close_method: CloseMethod,
        issuer: SchemaIssuer<Rgb25>,
        by: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(close_method, issuer, by, name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb25>>(
        close_method: CloseMethod,
        by: &str,
        name: &str,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        let mut me = Self::testnet_int(close_method, C::issuer(), by, name, precision)?;
        me.deterministic = true;
        Ok(me)
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

    pub fn allocate(mut self, outpoint: Outpoint, amount: Amount) -> Result<Self, IssuerError> {
        debug_assert!(
            !self.deterministic,
            "for creating deterministic contracts please use allocate_det method"
        );

        let beneficiary = GenesisSeal::new_random(outpoint.txid, outpoint.vout);
        self.issued
            .checked_add_assign(amount)
            .ok_or(IssuerError::AmountOverflow)?;
        self.builder =
            self.builder
                .add_fungible_state("assetOwner", beneficiary, amount.value())?;
        Ok(self)
    }

    pub fn allocate_all(
        mut self,
        allocations: impl IntoIterator<Item = (Outpoint, Amount)>,
    ) -> Result<Self, IssuerError> {
        for (beneficiary, amount) in allocations {
            self = self.allocate(beneficiary, amount)?;
        }
        Ok(self)
    }

    /// Add asset allocation in a deterministic way.
    pub fn allocate_det(
        mut self,
        outpoint: Outpoint,
        seal_blinding: u64,
        amount: Amount,
    ) -> Result<Self, IssuerError> {
        debug_assert!(
            self.deterministic,
            "to add asset allocation in deterministic way the contract builder has to be created \
             using `*_det` constructor"
        );

        let beneficiary = GenesisSeal::with_blinding(outpoint.txid, outpoint.vout, seal_blinding);
        self.issued
            .checked_add_assign(amount)
            .ok_or(IssuerError::AmountOverflow)?;
        self.builder = self.builder.add_owned_state_det(
            "assetOwner",
            beneficiary,
            PersistedState::Amount(amount),
        )?;
        Ok(self)
    }

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
