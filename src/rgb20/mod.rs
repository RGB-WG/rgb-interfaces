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
mod wrapper;
mod issuer;
mod info;

pub use info::{Rgb20Info, SupplyEvent, SupplyInfo};
pub use issuer::{IssuerError, PrimaryIssue};
pub use wrapper::{Rgb20, RGB20_FIXED_IFACE_ID, RGB20_IFACE_ID};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub enum Inflation {
    #[default]
    Fixed,
    Burnable,
    Inflatable,
    InflatableBurnable,
    Replaceable,
}

impl Inflation {
    pub fn is_fixed(self) -> bool { self == Self::Fixed }
    pub fn is_inflatable(self) -> bool {
        self == Self::Inflatable || self == Self::InflatableBurnable || self == Self::Replaceable
    }
    pub fn is_replaceable(self) -> bool { self == Self::Replaceable }
    pub fn is_burnable(self) -> bool { self == Self::Burnable || self == Self::Replaceable }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct Features {
    pub renaming: bool,
    // pub reserves: bool,
    pub inflation: Inflation,
}

impl Features {
    pub const NONE: Self = Features {
        renaming: false,
        // reserves: false,
        inflation: Inflation::Fixed,
    };
    pub const ALL: Self = Features {
        renaming: true,
        // reserves: true,
        inflation: Inflation::Replaceable,
    };
    pub const ENUMERATE: &'static [Self] = &[Self::NONE, Self::ALL];
}

#[cfg(test)]
mod test {
    use amplify::ByteArray;
    use rgbstd::interface::IfaceClass;

    use super::*;

    #[test]
    fn iface_id_all() {
        let iface_id = Rgb20::iface(Features::NONE).iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb20::IFACE_IDS[0], iface_id);
        let iface_id = Rgb20::iface(Features::ALL).iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb20::IFACE_IDS[1], iface_id);
    }

    #[test]
    fn iface_check() {
        if let Err(err) = Rgb20::iface(Features::NONE).check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB20Fixed interface definition");
        }
        if let Err(err) = Rgb20::iface(Features::ALL).check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB20 interface definition");
        }
    }
}
