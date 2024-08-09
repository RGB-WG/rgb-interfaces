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

use rgbstd::interface::{
    AssignIface, GenesisIface, GlobalIface, Iface, IfaceClass, Modifier, OwnedIface, Req,
    TransitionIface, VerNo,
};
use rgbstd::stl::StandardTypes;
use rgbstd::{Identity, Occurrences};

use super::Rgb21;
use crate::LNPBP_IDENTITY;

pub fn nft() -> Iface {
    let types = StandardTypes::with(Rgb21::ALL.stl());
    Iface {
        version: VerNo::V1,
        name: tn!("NonFungibleToken"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("tokens") => GlobalIface::none_or_many(types.get("RGB21.TokenData")),
            fname!("attachmentTypes") => GlobalIface::none_or_many(types.get("RGB21.AttachmentType")),
        },
        assignments: tiny_bmap! {
            fname!("assetOwner") => AssignIface::private(OwnedIface::Data(types.get("RGBContract.Allocation")), Req::NoneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("tokens") => Occurrences::NoneOrMore,
                fname!("attachmentTypes") => Occurrences::NoneOrMore,
            },
            assignments: tiny_bmap! {
                fname!("assetOwner") => Occurrences::NoneOrMore,
            },
            valencies: none!(),
            errors: tiny_bset! {
                vname!("unknownToken"),
                vname!("fractionOverflow"),
                vname!("invalidAttachmentType")
            },
        },
        transitions: tiny_bmap! {
            fname!("transfer") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: none!(),
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::OnceOrMore,
                },
                assignments: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::OnceOrMore,
                },
                valencies: none!(),
                errors: tiny_bset! {
                vname!("unknownToken"),
                    vname!("nonEqualValues"),
                    vname!("fractionOverflow"),
                    vname!("nonFractionalToken")
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        errors: tiny_bmap! {
            vname!("fractionOverflow")
                => tiny_s!("the amount of token fractions in outputs exceeds 1"),

            vname!("unknownToken")
                => tiny_s!("allocation of unknown token ID"),

            vname!("nonEqualValues")
                => tiny_s!("the sum of spent token fractions doesn't equal to the sum of token fractions in outputs"),

            vname!("nonFractionalToken")
                => tiny_s!("attempt to transfer a fraction of non-fractionable token"),

            vname!("invalidAttachmentType")
                => tiny_s!("attachment has a type which is not allowed for the token"),
        },
        default_operation: Some(fname!("transfer")),
    }
}

pub fn unique() -> Iface {
    let types = StandardTypes::with(Rgb21::ALL.stl());
    Iface {
        version: VerNo::V1,
        name: tn!("UniqueNft"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("tokens") => GlobalIface::required(types.get("RGB21.TokenData")),
            fname!("attachmentTypes") => GlobalIface::required(types.get("RGB21.AttachmentType")),
        },
        assignments: tiny_bmap! {
            fname!("assetOwner") => AssignIface::private(OwnedIface::Data(types.get("RGBContract.Allocation")), Req::OneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("tokens") => Occurrences::Once,
                fname!("attachmentTypes") => Occurrences::Once,
            },
            assignments: tiny_bmap! {
                fname!("assetOwner") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: none!(),
        extensions: none!(),
        errors: none!(),
        default_operation: None,
    }
}

pub fn limited() -> Iface {
    let types = StandardTypes::with(Rgb21::ALL.stl());
    Iface {
        version: VerNo::V1,
        name: tn!("LimitedNft"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("tokens") => GlobalIface::one_or_many(types.get("RGB21.TokenData")),
            fname!("attachmentTypes") => GlobalIface::one_or_many(types.get("RGB21.AttachmentType")),
        },
        assignments: tiny_bmap! {
            fname!("assetOwner") => AssignIface::private(OwnedIface::Data(types.get("RGBContract.Allocation")), Req::OneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("tokens") => Occurrences::OnceOrMore,
                fname!("attachmentTypes") => Occurrences::OnceOrMore,
            },
            assignments: tiny_bmap! {
                fname!("assetOwner") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: none!(),
        extensions: none!(),
        errors: none!(),
        default_operation: None,
    }
}

pub fn engravable() -> Iface {
    let types = StandardTypes::with(Rgb21::ALL.stl());
    Iface {
        version: VerNo::V1,
        name: tn!("EngravableNft"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("engravings") => GlobalIface::none_or_many(types.get("RGB21.EngravingData")),
        },
        assignments: none!(),
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: none!(),
            valencies: none!(),
            errors: none!(),
        },
        transitions: tiny_bmap! {
            fname!("engrave") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: none!(),
                globals: tiny_bmap! {
                    fname!("engravings") => Occurrences::Once,
                },
                inputs: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::OnceOrMore,
                },
                assignments: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::OnceOrMore,
                },
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("unknownToken"),
                    vname!("nonEqualValues"),
                    vname!("fractionOverflow"),
                    vname!("nonFractionalToken"),
                    vname!("nonEngravableToken")
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        errors: tiny_bmap! {
            vname!("nonEngravableToken")
                => tiny_s!("attempt to engrave on a token which prohibit engraving"),
        },
        default_operation: None,
    }
}

pub fn issuable() -> Iface {
    let types = StandardTypes::with(Rgb21::ALL.stl());
    Iface {
        version: VerNo::V1,
        name: tn!("IssuableNft"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: none!(),
        assignments: tiny_bmap! {
            fname!("inflationAllowance") => AssignIface::public(OwnedIface::Data(types.get("RGB21.ItemsCount")), Req::OneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: tiny_bmap! {
                fname!("inflationAllowance") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: tiny_bmap! {
            fname!("issue") => TransitionIface {
                modifier: Modifier::Abstract,
                optional: false,
                metadata: none!(),
                globals: tiny_bmap! {
                    fname!("tokens") => Occurrences::NoneOrMore,
                    fname!("attachmentTypes") => Occurrences::NoneOrMore,
                },
                inputs: tiny_bmap! {
                    fname!("inflationAllowance") => Occurrences::OnceOrMore,
                },
                assignments: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::NoneOrMore,
                    fname!("inflationAllowance") => Occurrences::NoneOrMore,
                },
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("unknownToken"),
                    vname!("fractionOverflow"),
                    vname!("invalidAttachmentType"),
                    vname!("issueExceedsAllowance"),
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        errors: tiny_bmap! {
            vname!("issueExceedsAllowance")
                => tiny_s!("you try to issue more assets than allowed by the contract terms"),
        },
        default_operation: None,
    }
}
