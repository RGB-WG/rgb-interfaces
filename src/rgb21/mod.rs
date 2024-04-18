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

pub use types::{
    AttachmentName, AttachmentType, EmbeddedMedia, EngravingData, ItemsCount, TokenData,
    LIB_ID_RGB21, LIB_NAME_RGB21,
};
pub use wrapper::{Rgb21, RGB21_UNIQUE_IFACE_ID};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Issues {
    #[default]
    Unique,
    Limited,
    MultiIssue,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Features {
    pub renaming: bool,
    pub reserves: bool,
    pub engraving: bool,
    pub issues: Issues,
}

impl Features {
    pub const NONE: Self = Features {
        renaming: false,
        reserves: false,
        engraving: false,
        issues: Issues::Unique,
    };
    pub const ALL: Self = Features {
        renaming: true,
        reserves: true,
        engraving: true,
        issues: Issues::MultiIssue,
    };

    pub const ENUMERATE: &'static [Self] = &[Self::NONE, Self::ALL];
}

#[cfg(test)]
mod test {
    use amplify::ByteArray;
    use rgbstd::interface::IfaceClass;

    use super::*;

    #[test]
    fn iface_id() {
        let iface_id = Rgb21::iface(Features::NONE).iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb21::IFACE_IDS[0], iface_id);
        let iface_id = Rgb21::iface(Features::ALL).iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb21::IFACE_IDS[1], iface_id);
    }

    #[test]
    fn iface_check() {
        if let Err(err) = Rgb21::iface(Features::NONE).check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB21Unique interface definition");
        }
        if let Err(err) = Rgb21::iface(Features::ALL).check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB21 interface definition");
        }
    }
}
