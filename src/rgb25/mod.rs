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
pub use info::Rgb25Info;
pub use issuer::Issue;
use rgbstd::info::FeatureList;
pub use wrapper::{Rgb25, RGB25_IFACE_ID};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", rename_all = "camelCase")
)]
pub struct Features {
    pub burnable: bool,
}

impl Features {
    pub const NONE: Self = Features { burnable: false };
    pub const ALL: Self = Features { burnable: true };

    pub const ENUMERATE: &'static [Self] = &[Self::NONE, Self::ALL];

    pub fn to_list(&self) -> FeatureList {
        let mut list = bset![fname!("fractional")];
        if self.burnable {
            list.insert(fname!("burnable"));
        }
        Confined::from_collection_unsafe(list).into()
    }
}

#[cfg(test)]
mod test {
    use amplify::ByteArray;
    use rgbstd::interface::IfaceClass;

    use super::*;

    #[test]
    fn iface_id_all() {
        let iface_id = Rgb25::iface(Features::NONE).iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb25::IFACE_IDS[0], iface_id);
        let iface_id = Rgb25::iface(Features::ALL).iface_id();
        eprintln!("{:#04x?}", iface_id.to_byte_array());
        assert_eq!(Rgb25::IFACE_IDS[1], iface_id);
    }

    #[test]
    fn iface_check() {
        if let Err(err) = Rgb25::iface(Features::NONE).check() {
            for e in err {
                eprintln!("- {e}");
            }
            panic!("invalid RGB25 interface definition");
        }
        if let Err(err) = Rgb25::iface(Features::ALL).check() {
            for e in err {
                eprintln!("- {e}");
            }
            panic!("invalid RGB25 interface definition");
        }
    }
}
