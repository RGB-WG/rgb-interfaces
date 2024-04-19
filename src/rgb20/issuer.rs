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
use rgbstd::stl::{AssetSpec, Attachment, ContractTerms, RicardianContract};
use rgbstd::{AltLayer1, AssetTag, BlindingFactor, GenesisSeal};
use strict_encoding::InvalidRString;

use super::Rgb20;
use crate::{IssuerWrapper, SchemaIssuer};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, Error)]
#[display(doc_comments)]
pub enum AllocationError {
    /// contract genesis doesn't support allocating to liquid seals; request
    /// liquid support first.
    NoLiquidSupport,
    /// overflow in the amount of the issued assets: the total amount must not
    /// exceed 2^64.
    AmountOverflow,
}

impl From<BuilderError> for AllocationError {
    fn from(err: BuilderError) -> Self {
        match err {
            BuilderError::InvalidLayer1(_) => AllocationError::NoLiquidSupport,
            _ => panic!("invalid RGB20 schema (assetOwner mismatch)"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrimaryIssue {
    builder: ContractBuilder,
    issued: Amount,
    terms: ContractTerms,
}

impl PrimaryIssue {
    pub fn testnet_with(
        issuer: SchemaIssuer<Rgb20>,
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(issuer, ticker, name, details, precision, false)
    }

    pub fn testnet<C: IssuerWrapper<IssuingIface = Rgb20>>(
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
    ) -> Result<Self, InvalidRString> {
        Self::testnet_int(C::issuer(), ticker, name, details, precision, false)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Rgb20>>(
        ticker: &str,
        name: &str,
        details: Option<&str>,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Self, InvalidRString> {
        let mut me = Self::testnet_int(C::issuer(), ticker, name, details, precision, true)?;
        me.builder = me
            .builder
            .add_asset_tag("assetOwner", asset_tag)
            .expect("invalid RGB20 schema (assetOwner mismatch)");
        Ok(me)
    }

    fn testnet_int(
        issuer: SchemaIssuer<Rgb20>,
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
                Rgb20::iface(features),
                schema,
                main_iface_impl,
                types,
                scripts,
            ),
            true => ContractBuilder::deterministic(
                Rgb20::iface(features),
                schema,
                main_iface_impl,
                types,
                scripts,
            ),
        };
        builder = builder
            .add_global_state("spec", spec)
            .expect("invalid RGB20 schema (token specification mismatch)");

        Ok(Self {
            builder,
            terms,
            issued: Amount::ZERO,
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

    pub fn allocate<O: TxOutpoint>(
        mut self,
        method: Method,
        beneficiary: O,
        amount: Amount,
    ) -> Result<Self, AllocationError> {
        let beneficiary = beneficiary.map_to_xchain(|outpoint| {
            GenesisSeal::new_random(method, outpoint.txid, outpoint.vout)
        });
        self.issued
            .checked_add_assign(amount)
            .ok_or(AllocationError::AmountOverflow)?;
        self.builder =
            self.builder
                .add_fungible_state("assetOwner", beneficiary, amount.value())?;
        Ok(self)
    }

    pub fn allocate_all<O: TxOutpoint>(
        mut self,
        method: Method,
        allocations: impl IntoIterator<Item = (O, Amount)>,
    ) -> Result<Self, AllocationError> {
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
    ) -> Result<Self, AllocationError> {
        let beneficiary = beneficiary.map_to_xchain(|outpoint| {
            GenesisSeal::with_blinding(method, outpoint.txid, outpoint.vout, seal_blinding)
        });
        self.issued
            .checked_add_assign(amount)
            .ok_or(AllocationError::AmountOverflow)?;
        self.builder = self.builder.add_fungible_state_det(
            "assetOwner",
            beneficiary,
            amount,
            amount_blinding,
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
            .expect("invalid RGB20 schema (issued supply mismatch)")
            .add_global_state("terms", self.terms)
            .expect("invalid RGB20 schema (contract terms mismatch)")
    }

    // TODO: Add secondary issuance and other methods
}
