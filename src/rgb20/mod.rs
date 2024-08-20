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

use amplify::confinement::Confined;
use rgbstd::info::FeatureList;
use rgbstd::interface::{Iface, IfaceClass, IfaceId};
use rgbstd::persistence::ContractStateRead;
use rgbstd::stl::rgb_contract_stl;
use strict_types::TypeLib;

use self::iface::{burnable, fixed, inflatable, replaceable, rgb20_base, rgb20_renamable};
pub use self::info::{Rgb20Info, SupplyEvent, SupplyInfo};
pub use self::issuer::{IssuerError, PrimaryIssue};
pub use self::wrapper::{Rgb20Wrapper, RGB20_FIXED_IFACE_ID, RGB20_FULL_IFACE_ID};
use crate::rgb20::wrapper::{
    RGB20_BURNABLE_IFACE_ID, RGB20_INFLATABLE_BURNABLE_IFACE_ID, RGB20_INFLATABLE_IFACE_ID,
    RGB20_RENAMABLE_BURNABLE_IFACE_ID, RGB20_RENAMABLE_IFACE_ID,
    RGB20_RENAMABLE_INFLATABLE_BURNABLE_IFACE_ID, RGB20_RENAMABLE_INFLATABLE_IFACE_ID,
    RGB20_REPLACABLE_IFACE_ID,
};

pub const LIB_NAME_RGB20: &str = "RGB20";

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Display)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
#[display(lowercase)]
pub enum Inflation {
    #[default]
    Fixed,
    Burnable,
    Inflatable,
    #[display("inflatable, burnable")]
    InflatableBurnable,
    Replaceable,
}

impl Inflation {
    pub fn is_fixed(self) -> bool { self == Self::Fixed }
    pub fn is_inflatable(self) -> bool {
        self == Self::Inflatable || self == Self::InflatableBurnable || self == Self::Replaceable
    }
    pub fn is_replaceable(self) -> bool { self == Self::Replaceable }
    pub fn is_burnable(self) -> bool {
        self == Self::Burnable || self == Self::Replaceable || self == Self::InflatableBurnable
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct Rgb20 {
    pub renaming: bool,
    // pub reserves: bool,
    pub inflation: Inflation,
}

impl Rgb20 {
    pub const FIXED: Self = Rgb20 {
        renaming: false,
        // reserves: false,
        inflation: Inflation::Fixed,
    };
    pub const RENAMABLE: Self = Rgb20 {
        renaming: true,
        // reserves: false,
        inflation: Inflation::Fixed,
    };
    pub const INFLATABLE: Self = Rgb20 {
        renaming: false,
        // reserves: false,
        inflation: Inflation::Inflatable,
    };
    pub const BURNABLE: Self = Rgb20 {
        renaming: false,
        // reserves: false,
        inflation: Inflation::Burnable,
    };
    pub const INFLATABLE_BURNABLE: Self = Rgb20 {
        renaming: false,
        // reserves: false,
        inflation: Inflation::InflatableBurnable,
    };
    pub const REPLACEABLE: Self = Rgb20 {
        renaming: false,
        // reserves: false,
        inflation: Inflation::Replaceable,
    };
    pub const RENAMABLE_INFLATABLE: Self = Rgb20 {
        renaming: true,
        // reserves: false,
        inflation: Inflation::Inflatable,
    };
    pub const RENAMABLE_BURNABLE: Self = Rgb20 {
        renaming: true,
        // reserves: false,
        inflation: Inflation::Burnable,
    };
    pub const RENAMABLE_INFLATABLE_BURNABLE: Self = Rgb20 {
        renaming: true,
        // reserves: false,
        inflation: Inflation::InflatableBurnable,
    };
    pub const ALL: Self = Rgb20 {
        renaming: true,
        // reserves: true,
        inflation: Inflation::Replaceable,
    };
    pub const ENUMERATE: &'static [Self] = &[
        Self::FIXED,
        Self::RENAMABLE,
        Self::INFLATABLE,
        Self::BURNABLE,
        Self::INFLATABLE_BURNABLE,
        Self::REPLACEABLE,
        Self::RENAMABLE_INFLATABLE,
        Self::RENAMABLE_BURNABLE,
        Self::RENAMABLE_INFLATABLE_BURNABLE,
        Self::ALL,
    ];

    pub fn to_list(&self) -> FeatureList {
        let mut list = bset![];
        if self.renaming {
            list.insert(fname!("renamable"));
        }
        if self.inflation.is_burnable() {
            list.insert(fname!("burnable"));
        }
        if self.inflation.is_inflatable() {
            list.insert(fname!("inflatable"));
        }
        if self.inflation.is_replaceable() {
            list.insert(fname!("replaceable"));
        }
        Confined::from_checked(list).into()
    }
}

impl IfaceClass for Rgb20 {
    const IFACE_NAME: &'static str = LIB_NAME_RGB20;
    const IFACE_IDS: &'static [IfaceId] = &[
        RGB20_FIXED_IFACE_ID,
        RGB20_RENAMABLE_IFACE_ID,
        RGB20_INFLATABLE_IFACE_ID,
        RGB20_BURNABLE_IFACE_ID,
        RGB20_INFLATABLE_BURNABLE_IFACE_ID,
        RGB20_REPLACABLE_IFACE_ID,
        RGB20_RENAMABLE_INFLATABLE_IFACE_ID,
        RGB20_RENAMABLE_BURNABLE_IFACE_ID,
        RGB20_RENAMABLE_INFLATABLE_BURNABLE_IFACE_ID,
        RGB20_FULL_IFACE_ID,
    ];
    type Wrapper<S: ContractStateRead> = Rgb20Wrapper<S>;

    fn iface(&self) -> Iface {
        let (mut name, mut iface) = if self.renaming {
            (tn!("RGB20Renamable"), rgb20_renamable())
        } else {
            (tn!("RGB20"), rgb20_base())
        };
        if self.inflation.is_fixed() {
            iface = iface.expect_extended(fixed(), tn!(format!("{name}Fixed")));
        } else if self.inflation.is_inflatable() {
            iface = iface.expect_extended(inflatable(), tn!(format!("{name}Inflatable")));
            name = tn!(format!("{name}Inflatable"));
        }
        if self.inflation.is_burnable() {
            iface = iface.expect_extended(burnable(), tn!(format!("{name}Burnable")));
            if self.inflation.is_replaceable() {
                name = tn!(format!("{}Replaceable", name.to_string().replace("Inflatable", "")));
                iface = iface.expect_extended(replaceable(), name);
            }
        }
        /* TODO: Complete reservable interface
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB20Reservable");
        }
         */
        iface
    }

    fn iface_id(&self) -> IfaceId {
        // TODO: Optimize with constants
        self.iface().iface_id()
    }

    fn stl(&self) -> TypeLib { rgb_contract_stl() }
}

mod _display {
    use std::fmt::{self, Display, Formatter};

    use super::*;

    impl Display for Rgb20 {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            if self.renaming {
                f.write_str("renamable, ")?;
            }
            Display::fmt(&self.inflation, f)
        }
    }
}

#[cfg(test)]
mod test {
    use amplify::ByteArray;
    use rgbstd::interface::IfaceClass;

    use super::*;

    #[test]
    fn iface_id_all() {
        for (iface, iface_id) in Rgb20::ENUMERATE.iter().zip(Rgb20::IFACE_IDS) {
            if iface.iface_id() != *iface_id {
                eprintln!("{}: {:#04x?}", iface.iface().name, iface.iface_id().to_byte_array());
            }
            assert_eq!(iface.iface_id(), *iface_id);
        }
    }

    #[test]
    fn iface_check() {
        if let Err(err) = Rgb20::FIXED.iface().check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB20Fixed interface definition");
        }
        if let Err(err) = Rgb20::ALL.iface().check() {
            for e in err {
                eprintln!("{e}");
            }
            panic!("invalid RGB20 interface definition");
        }
    }
}
