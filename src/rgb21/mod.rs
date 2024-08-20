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

pub mod iface;
mod types;
mod wrapper;

use amplify::confinement::Confined;
use rgbstd::info::FeatureList;
use rgbstd::interface::{Iface, IfaceClass, IfaceId};
use rgbstd::persistence::ContractStateRead;
use strict_types::TypeLib;

use self::iface::{engravable, issuable, limited, nft, unique};
pub use self::types::{
    AttachmentName, AttachmentType, EmbeddedMedia, EngravingData, ItemsCount, TokenData,
    LIB_ID_RGB21, LIB_NAME_RGB21,
};
pub use self::wrapper::{Rgb21Wrapper, RGB21_IFACE_ID, RGB21_UNIQUE_IFACE_ID};
use crate::rgb20::iface::{named_asset, renameable};
use crate::rgb21::wrapper::rgb21_stl;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Issues {
    #[default]
    Unique,
    Limited,
    MultiIssue,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Rgb21 {
    pub renaming: bool,
    // pub reserves: bool,
    pub engraving: bool,
    pub issues: Issues,
}

impl Rgb21 {
    pub const NONE: Self = Rgb21 {
        renaming: false,
        // reserves: false,
        engraving: false,
        issues: Issues::Unique,
    };
    pub const ALL: Self = Rgb21 {
        renaming: true,
        // reserves: true,
        engraving: true,
        issues: Issues::MultiIssue,
    };

    pub const ENUMERATE: &'static [Self] = &[Self::NONE, Self::ALL];

    pub fn to_list(&self) -> FeatureList {
        let mut list = bset![fname!("fractional")];
        if self.renaming {
            list.insert(fname!("renamable"));
        }
        if self.engraving {
            list.insert(fname!("engravable"));
        }
        match self.issues {
            Issues::Unique => list.insert(fname!("unique")),
            Issues::Limited => list.insert(fname!("limited")),
            Issues::MultiIssue => list.insert(fname!("collection")),
        };
        Confined::from_checked(list).into()
    }
}

impl IfaceClass for Rgb21 {
    const IFACE_NAME: &'static str = LIB_NAME_RGB21;
    const IFACE_IDS: &'static [IfaceId] = &[RGB21_UNIQUE_IFACE_ID, RGB21_IFACE_ID];
    type Wrapper<S: ContractStateRead> = Rgb21Wrapper<S>;

    fn iface(&self) -> Iface {
        let mut iface = named_asset().expect_extended(nft(), "RGB21Base");
        if self.renaming {
            iface = iface.expect_extended(renameable(), "RGB21Renameable");
        }
        if self.engraving {
            iface = iface.expect_extended(engravable(), "RGB21Engravable");
        }
        iface = match self.issues {
            Issues::Unique => iface.expect_extended(unique(), "RGB21Unique"),
            Issues::Limited => iface.expect_extended(limited(), "RGB21Limited"),
            Issues::MultiIssue => iface.expect_extended(issuable(), "RGB21Issuable"),
        };
        /*
        if self.reserves {
            iface = iface.expect_extended(reservable(), "RGB21Reservable");
        }
         */
        iface
    }

    fn iface_id(&self) -> IfaceId {
        // TODO: Optimize with constants
        self.iface().iface_id()
    }

    fn stl(&self) -> TypeLib { rgb21_stl() }
}

#[cfg(test)]
mod test {
    use amplify::ByteArray;
    use rgbstd::interface::IfaceClass;

    use super::*;

    #[test]
    fn iface_id() {
        let iface_id = Rgb21::NONE.iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb21::IFACE_IDS[0], iface_id);
        let iface_id = Rgb21::ALL.iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb21::IFACE_IDS[1], iface_id);
    }

    #[test]
    fn iface_check() {
        if let Err(err) = Rgb21::NONE.iface().check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB21Unique interface definition");
        }
        if let Err(err) = Rgb21::ALL.iface().check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB21 interface definition");
        }
    }
}
