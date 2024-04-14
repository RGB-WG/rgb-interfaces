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

use std::marker::PhantomData;

use amplify::confinement::Confined;
use rgbstd::containers::{ContainerVer, Kit, ValidKit};
use rgbstd::interface::{ContractIface, Iface, IfaceId, IfaceImpl};
use rgbstd::validation::Scripts;
use rgbstd::Schema;
use strict_types::typesys::UnknownType;
use strict_types::{TypeLib, TypeSystem};

pub trait IfaceWrapper: From<ContractIface> {
    const IFACE_NAME: &'static str;
    const IFACE_IDS: &'static [IfaceId];

    type Features: Sized + Clone + Default;

    fn iface(features: Self::Features) -> Iface;
    fn stl() -> TypeLib;
}

pub trait IssuerWrapper {
    const FEATURES: <Self::IssuingIface as IfaceWrapper>::Features;
    type IssuingIface: IfaceWrapper;

    fn schema() -> Schema;
    fn issue_impl() -> IfaceImpl;
    fn types() -> TypeSystem;
    fn scripts() -> Scripts;

    fn issuer() -> SchemaIssuer<Self::IssuingIface> {
        SchemaIssuer::new(
            Self::schema(),
            Self::issue_impl(),
            Self::types(),
            Self::scripts(),
            Self::FEATURES,
        )
        .expect("wrong Self::types implementation")
    }

    fn kit() -> ValidKit {
        let kit = Kit {
            version: ContainerVer::V2,
            ifaces: tiny_bset![Self::IssuingIface::iface(Self::FEATURES)],
            schemata: tiny_bset![Self::schema()],
            iimpls: tiny_bset![Self::issue_impl()],
            supplements: none!(),
            types: Self::types(),
            scripts: Confined::from_iter_unsafe(Self::scripts().into_inner().into_values()),
            signatures: none!(),
        };
        kit.validate().expect("invalid construction")
    }
}

#[derive(Getters, Clone, Eq, PartialEq, Debug)]
pub struct SchemaIssuer<I: IfaceWrapper> {
    schema: Schema,
    iimpl: IfaceImpl,
    features: I::Features,
    types: TypeSystem,
    scripts: Scripts,
    phantom: PhantomData<I>,
}

impl<I: IfaceWrapper> SchemaIssuer<I> {
    #[allow(clippy::result_large_err)]
    pub fn new(
        schema: Schema,
        iimpl: IfaceImpl,
        type_system: TypeSystem,
        scripts: Scripts,
        features: I::Features,
    ) -> Result<Self, UnknownType> {
        let types = type_system.extract(schema.types())?;
        Ok(Self {
            schema,
            iimpl,
            features,
            types,
            scripts,
            phantom: default!(),
        })
    }

    #[inline]
    pub fn into_split(self) -> (Schema, IfaceImpl, TypeSystem, Scripts, I::Features) {
        (self.schema, self.iimpl, self.types, self.scripts, self.features)
    }
}
