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

use std::borrow::Borrow;
use std::iter;

use amplify::num::u24;
use rgbstd::persistence::ContractStateRead;
use rgbstd::vm::{
    ContractState, GlobalContractState, GlobalOrd, GlobalStateIter, UnknownGlobalStateType,
};
use rgbstd::{
    AssignmentType, AttachState, ContractId, DataState, FungibleState, GlobalStateType,
    OutputAssignment, RevealedAttach, RevealedData, RevealedValue, SchemaId, VoidState, XOutpoint,
};

// TODO: Get rid of Dumb type
pub struct Dumb;
impl GlobalStateIter for Dumb {
    type Data = DataState;
    fn size(&mut self) -> u24 { unreachable!() }
    fn prev(&mut self) -> Option<(GlobalOrd, Self::Data)> { unreachable!() }
    fn last(&mut self) -> Option<(GlobalOrd, Self::Data)> { unreachable!() }
    fn reset(&mut self, _: u24) { unreachable!() }
}
impl ContractState for Dumb {
    fn global(
        &self,
        _: GlobalStateType,
    ) -> Result<GlobalContractState<impl GlobalStateIter>, UnknownGlobalStateType> {
        unreachable!();
        #[allow(unreachable_code)]
        Ok(GlobalContractState::new(Dumb))
    }
    fn rights(&self, _: XOutpoint, _: AssignmentType) -> u32 { unreachable!() }
    fn fungible(
        &self,
        _: XOutpoint,
        _: AssignmentType,
    ) -> impl DoubleEndedIterator<Item = FungibleState> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty()
    }
    fn data(
        &self,
        _: XOutpoint,
        _: AssignmentType,
    ) -> impl DoubleEndedIterator<Item = impl Borrow<DataState>> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty::<DataState>()
    }
    fn attach(
        &self,
        _: XOutpoint,
        _: AssignmentType,
    ) -> impl DoubleEndedIterator<Item = impl Borrow<AttachState>> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty::<AttachState>()
    }
}
impl ContractStateRead for Dumb {
    fn contract_id(&self) -> ContractId { unreachable!() }
    fn schema_id(&self) -> SchemaId { unreachable!() }
    fn rights_all(&self) -> impl Iterator<Item = &OutputAssignment<VoidState>> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty()
    }
    fn fungible_all(&self) -> impl Iterator<Item = &OutputAssignment<RevealedValue>> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty()
    }
    fn data_all(&self) -> impl Iterator<Item = &OutputAssignment<RevealedData>> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty()
    }
    fn attach_all(&self) -> impl Iterator<Item = &OutputAssignment<RevealedAttach>> {
        unreachable!();
        #[allow(unreachable_code)]
        iter::empty()
    }
}
