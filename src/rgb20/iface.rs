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
    AssignIface, GenesisIface, GlobalIface, Iface, Modifier, OwnedIface, Req, TransitionIface,
    VerNo,
};
use rgbstd::schema::Occurrences;
use rgbstd::stl::StandardTypes;
use rgbstd::Identity;

use crate::LNPBP_IDENTITY;

pub fn rgb20_base() -> Iface { named_asset().expect_extended(fungible(), tn!("RGB20Base")) }
pub fn rgb20_renamable() -> Iface {
    rgb20_base().expect_extended(renameable(), tn!("RGB20Renamable"))
}

pub fn named_asset() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        name: tn!("NamedAsset"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("spec") => GlobalIface::required(types.get("RGBContract.AssetSpec")),
            fname!("terms") => GlobalIface::required(types.get("RGBContract.ContractTerms")),
        },
        assignments: none!(),
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Abstract,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("spec") => Occurrences::Once,
                fname!("terms") => Occurrences::Once,
            },
            assignments: none!(),
            valencies: none!(),
            errors: none!(),
        },
        transitions: none!(),
        extensions: none!(),
        errors: none!(),
        default_operation: None,
    }
}

pub fn renameable() -> Iface {
    Iface {
        version: VerNo::V1,
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        name: tn!("RenameableAsset"),
        metadata: none!(),
        global_state: none!(),
        assignments: tiny_bmap! {
            fname!("updateRight") => AssignIface::public(OwnedIface::Rights, Req::Required),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: tiny_bmap! {
                fname!("updateRight") => Occurrences::Once,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: tiny_bmap! {
            fname!("rename") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: none!(),
                globals: tiny_bmap! {
                    fname!("spec") => Occurrences::Once,
                },
                inputs: tiny_bmap! {
                    fname!("updateRight") => Occurrences::Once,
                },
                assignments: tiny_bmap! {
                    fname!("updateRight") => Occurrences::NoneOrOnce,
                },
                valencies: none!(),
                errors: none!(),
                default_assignment: Some(fname!("updateRight")),
            },
        },
        extensions: none!(),
        default_operation: None,
        errors: none!(),
    }
}

pub fn fungible() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        name: tn!("FungibleAsset"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("issuedSupply") => GlobalIface::required(types.get("RGBContract.Amount")),
        },
        assignments: tiny_bmap! {
            fname!("assetOwner") => AssignIface::private(OwnedIface::Amount, Req::NoneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("issuedSupply") => Occurrences::Once,
            },
            assignments: tiny_bmap! {
                fname!("assetOwner") => Occurrences::NoneOrMore,
            },
            valencies: none!(),
            errors: tiny_bset! {
                vname!("issuedMismatch"),
            },
        },
        transitions: tiny_bmap! {
            fname!("transfer") => TransitionIface {
                modifier: Modifier::Abstract,
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
                    vname!("nonEqualAmounts")
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        errors: tiny_bmap! {
            vname!("issuedMismatch")
                => tiny_s!("supply specified as a global parameter doesn't match the issued supply allocated to the asset owners"),

            vname!("nonEqualAmounts")
                => tiny_s!("the sum of spent assets doesn't equal to the sum of assets in outputs"),
        },
        default_operation: Some(fname!("transfer")),
    }
}

/*
pub fn reservable() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        name: tn!("ReservableAsset"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: tiny_bmap! {
            fname!("reserveProof") => types.get("RGBContract.IssueMeta"),
        },
        global_state: none!(),
        assignments: none!(),
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: tiny_bset![fname!("reserveProof")],
            globals: none!(),
            assignments: none!(),
            valencies: none!(),
            errors: tiny_bset! {
                vname!("invalidProof"),
                vname!("insufficientReserves")
            },
        },
        transitions: tiny_bmap! {
            fname!("issue") => TransitionIface {
                modifier: Modifier::Override,
                optional: true,
                metadata: tiny_bset![fname!("reserveProof")],
                globals: none!(),
                inputs: none!(),
                assignments: none!(),
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("invalidReservesProof"),
                    vname!("insufficientReserves")
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        errors: tiny_bmap! {
            vname!("invalidReservesProof")
                => tiny_s!("the provided proof of reserves is invalid"),

            vname!("insufficientReserves")
                => tiny_s!("reserve is insufficient to cover the issued assets"),
        },
        default_operation: None,
    }
}
 */

pub fn fixed() -> Iface {
    Iface {
        version: VerNo::V1,
        name: tn!("FixedAsset"),
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        metadata: none!(),
        global_state: none!(),
        assignments: tiny_bmap! {
            fname!("assetOwner") => AssignIface::private(OwnedIface::Amount, Req::OneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: tiny_bmap! {
                fname!("assetOwner") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: tiny_bset! {
                vname!("issuedMismatch"),
            },
        },
        transitions: none!(),
        extensions: none!(),
        errors: none!(),
        default_operation: None,
    }
}

pub fn inflatable() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        name: tn!("InflatableAsset"),
        metadata: tiny_bmap! {
            fname!("allowedInflation") => types.get("RGBContract.Amount"),
        },
        global_state: tiny_bmap! {
            fname!("issuedSupply") => GlobalIface::one_or_many(types.get("RGBContract.Amount")),
            fname!("maxSupply") => GlobalIface::required(types.get("RGBContract.Amount")),
        },
        assignments: tiny_bmap! {
            fname!("inflationAllowance") => AssignIface::public(OwnedIface::Amount, Req::NoneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: tiny_bmap! {
                fname!("maxSupply") => Occurrences::Once,
            },
            assignments: tiny_bmap! {
                fname!("inflationAllowance") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: tiny_bset! {
                vname!("issuedMismatch"),
                vname!("inflationMismatch"),
            },
        },
        transitions: tiny_bmap! {
            fname!("issue") => TransitionIface {
                modifier: Modifier::Abstract,
                optional: false,
                metadata: tiny_bset![fname!("allowedInflation")],
                globals: tiny_bmap! {
                    fname!("issuedSupply") => Occurrences::Once,
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
                    vname!("issuedMismatch"),
                    vname!("inflationExceedsAllowance"),
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        default_operation: None,
        errors: tiny_bmap! {
            vname!("inflationMismatch")
                => tiny_s!("reported sum of issued assets and inflation allowance doesn't match specified maximum supply"),
            vname!("inflationExceedsAllowance")
                => tiny_s!("you try to issue more assets than allowed by the contract terms"),
        },
    }
}

pub fn burnable() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        name: tn!("BurnableAsset"),
        metadata: tiny_bmap! {
            fname!("burnProof") => types.get("RGBContract.BurnMeta"),
        },
        global_state: tiny_bmap! {
            fname!("burnConsignmentUrl") => GlobalIface::optional(types.get("RGBContract.Details")),
            fname!("burnedSupply") => GlobalIface::none_or_many(types.get("RGBContract.Amount")),
        },
        assignments: tiny_bmap! {
            fname!("burnRight") => AssignIface::public(OwnedIface::Rights, Req::OneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: tiny_bmap! {
                fname!("burnRight") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: tiny_bmap! {
            fname!("burn") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: tiny_bset![fname!("burnProof")],
                globals: tiny_bmap! {
                    fname!("burnConsignmentUrl") => Occurrences::NoneOrOnce,
                    fname!("burnedSupply") => Occurrences::Once,
                },
                inputs: tiny_bmap! {
                    fname!("burnRight") => Occurrences::OnceOrMore,
                },
                assignments: tiny_bmap! {
                    fname!("burnRight") => Occurrences::NoneOrMore,
                },
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("invalidBurnProof")
                },
                default_assignment: None,
            },
        },
        extensions: none!(),
        default_operation: None,
        errors: tiny_bmap! {
            vname!("invalidBurnProof")
                => tiny_s!("the provided proof of reserves is invalid")
        },
    }
}

pub fn replaceable() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        name: tn!("ReplaceableAsset"),
        metadata: none!(),
        global_state: tiny_bmap! {
            fname!("replacedSupply") => GlobalIface::none_or_many(types.get("RGBContract.Amount")),
        },
        assignments: tiny_bmap! {
            fname!("replaceRight") => AssignIface::public(OwnedIface::Rights, Req::OneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: tiny_bmap! {
                fname!("replaceRight") => Occurrences::OnceOrMore,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: tiny_bmap! {
            fname!("replace") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: tiny_bset![fname!("burnProof")],
                globals: tiny_bmap! {
                    fname!("burnConsignmentUrl") => Occurrences::NoneOrOnce,
                    fname!("replacedSupply") => Occurrences::Once,
                },
                inputs: tiny_bmap! {
                    fname!("replaceRight") => Occurrences::OnceOrMore,
                },
                assignments: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::OnceOrMore,
                    fname!("replaceRight") => Occurrences::NoneOrOnce,
                },
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("issuedMismatch"),
                    vname!("invalidBurnProof"),
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        default_operation: None,
        errors: none!(),
    }
}

/*
pub fn replaceable_epochs() -> Iface {
    let types = StandardTypes::new();
    Iface {
        version: VerNo::V1,
        inherits: none!(),
        developer: Identity::from(LNPBP_IDENTITY),
        timestamp: 1711405444,
        name: tn!("ReplaceableAsset"),
        metadata: tiny_bmap! {
            fname!("burnProof") => types.get("RGBContract.BurnMeta"),
        },
        global_state: tiny_bmap! {
            fname!("burnedSupply") => GlobalIface::none_or_many(types.get("RGBContract.Amount")),
            fname!("replacedSupply") => GlobalIface::none_or_many(types.get("RGBContract.Amount")),
        },
        assignments: tiny_bmap! {
            fname!("burnEpoch") => AssignIface::public(OwnedIface::Rights, Req::OneOrMore),
            fname!("burnRight") => AssignIface::public(OwnedIface::Rights, Req::NoneOrMore),
        },
        valencies: none!(),
        genesis: GenesisIface {
            modifier: Modifier::Override,
            metadata: none!(),
            globals: none!(),
            assignments: tiny_bmap! {
                fname!("burnEpoch") => Occurrences::Once,
            },
            valencies: none!(),
            errors: none!(),
        },
        transitions: tiny_bmap! {
            fname!("openEpoch") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: none!(),
                globals: none!(),
                inputs: tiny_bmap! {
                    fname!("burnEpoch") => Occurrences::Once,
                },
                assignments: tiny_bmap! {
                    fname!("burnEpoch") => Occurrences::NoneOrOnce,
                    fname!("burnRight") => Occurrences::Once,
                },
                valencies: none!(),
                errors: none!(),
                default_assignment: Some(fname!("burnRight")),
            },
            fname!("burn") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: tiny_bset![fname!("burnProof")],
                globals: tiny_bmap! {
                    fname!("burnedSupply") => Occurrences::Once,
                },
                inputs: tiny_bmap! {
                    fname!("burnRight") => Occurrences::Once,
                },
                assignments: tiny_bmap! {
                    fname!("burnRight") => Occurrences::NoneOrOnce,
                },
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("issuedMismatch"),
                    vname!("invalidProof"),
                    vname!("insufficientCoverage")
                },
                default_assignment: None,
            },
            fname!("replace") => TransitionIface {
                modifier: Modifier::Final,
                optional: false,
                metadata: tiny_bset![fname!("burnProof")],
                globals: tiny_bmap! {
                    fname!("replacedSupply") => Occurrences::Once,
                },
                inputs: tiny_bmap! {
                    fname!("burnRight") => Occurrences::Once,
                },
                assignments: tiny_bmap! {
                    fname!("assetOwner") => Occurrences::NoneOrMore,
                    fname!("burnRight") => Occurrences::NoneOrOnce,
                },
                valencies: none!(),
                errors: tiny_bset! {
                    vname!("nonEqualAmounts"),
                    vname!("issuedMismatch"),
                    vname!("invalidProof"),
                    vname!("insufficientCoverage")
                },
                default_assignment: Some(fname!("assetOwner")),
            },
        },
        extensions: none!(),
        default_operation: None,
        errors: tiny_bmap! {
            vname!("insufficientCoverage")
                => tiny_s!("the claimed amount of burned assets is not covered by the assets in the operation inputs"),
        },
    }
}
*/
