// RGB schemata by LNP/BP Standards Association
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023 by
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

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

use ifaces::{rgb20, rgb21, rgb25, Dumb, Rgb20, Rgb21, Rgb25, LNPBP_IDENTITY};
use rgbstd::containers::{
    FileContent, Kit, Supplement, SUPPL_ANNOT_IFACE_CLASS, SUPPL_ANNOT_IFACE_FEATURES,
};
use rgbstd::info::IfaceClassName;
use rgbstd::interface::IfaceClass;
use rgbstd::stl::{bp_tx_stl, rgb_contract_stl};
use strict_types::stl::std_stl;
use strict_types::{StlFormat, SystemBuilder};

fn main() -> io::Result<()> {
    let ifsys = SystemBuilder::new()
        .import(Rgb21::<Dumb>::stl())
        .unwrap()
        .import(rgb_contract_stl())
        .unwrap()
        .import(bp_tx_stl())
        .unwrap()
        .import(std_stl())
        .unwrap()
        .finalize()
        .expect("not all libraries present");
    let typesys = ifsys.clone().into_type_system();

    let ifaces = [
        rgb20::iface::named_asset(),
        rgb20::iface::renameable(),
        rgb20::iface::fungible(),
        rgb20::iface::fixed(),
        rgb20::iface::burnable(),
        rgb20::iface::inflatable(),
        rgb20::iface::replaceable(),
        rgb21::iface::nft(),
        rgb21::iface::engravable(),
        rgb21::iface::unique(),
        rgb21::iface::limited(),
        rgb21::iface::issuable(),
        rgb25::iface::named_contract(),
    ];

    let mut kit = Kit::default();
    for iface in &ifaces {
        let types = typesys.extract(iface.types()).unwrap();
        kit.ifaces.push(iface.clone()).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGBStd.rgb")?;
    kit.save_armored("interfaces/RGBStd.rgba")?;

    let mut kit = Kit::default();
    for features in rgb20::Features::ENUMERATE {
        let iface = Rgb20::<Dumb>::iface(*features);
        let types = typesys.extract(iface.types()).unwrap();
        let mut suppl = Supplement::new(iface.iface_id(), LNPBP_IDENTITY);
        suppl
            .annotate_itself(SUPPL_ANNOT_IFACE_CLASS, &IfaceClassName::from("RGB20"))
            .unwrap();
        suppl
            .annotate_itself(SUPPL_ANNOT_IFACE_FEATURES, &features.to_list())
            .unwrap();
        kit.supplements.push(suppl).unwrap();
        kit.ifaces.push(iface).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGB20.rgb")?;
    kit.save_armored("interfaces/RGB20.rgba")?;

    let mut kit = Kit::default();
    for features in rgb21::Features::ENUMERATE {
        let iface = Rgb21::<Dumb>::iface(*features);
        let types = typesys.extract(iface.types()).unwrap();
        let mut suppl = Supplement::new(iface.iface_id(), LNPBP_IDENTITY);
        suppl
            .annotate_itself(SUPPL_ANNOT_IFACE_CLASS, &IfaceClassName::from("RGB21"))
            .unwrap();
        suppl
            .annotate_itself(SUPPL_ANNOT_IFACE_FEATURES, &features.to_list())
            .unwrap();
        kit.supplements.push(suppl).unwrap();
        kit.ifaces.push(iface).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGB21.rgb")?;
    kit.save_armored("interfaces/RGB21.rgba")?;

    let mut kit = Kit::default();
    for features in rgb25::Features::ENUMERATE {
        let iface = Rgb25::<Dumb>::iface(*features);
        let types = typesys.extract(iface.types()).unwrap();
        let mut suppl = Supplement::new(iface.iface_id(), LNPBP_IDENTITY);
        suppl
            .annotate_itself(SUPPL_ANNOT_IFACE_CLASS, &IfaceClassName::from("RGB25"))
            .unwrap();
        suppl
            .annotate_itself(SUPPL_ANNOT_IFACE_FEATURES, &features.to_list())
            .unwrap();
        kit.supplements.push(suppl).unwrap();
        kit.ifaces.push(iface).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGB25.rgb")?;
    kit.save_armored("interfaces/RGB25.rgba")?;

    let dir = PathBuf::from_str("interfaces").unwrap();
    let stl = Rgb21::<Dumb>::stl();
    stl.serialize(StlFormat::Binary, Some(&dir), "0.11.0", None)
        .expect("unable to write to the file");
    stl.serialize(StlFormat::Armored, Some(&dir), "0.11.0", None)
        .expect("unable to write to the file");
    stl.serialize(
        StlFormat::Source,
        Some(&dir),
        "0.11.0",
        Some(
            "
  Description: Types for RGB21 interfaces
  Author: Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
  Copyright (C) 2023-2024 LNP/BP Standards Association. All rights reserved.
  License: Apache-2.0",
        ),
    )
    .expect("unable to write to the file");

    let mut filename = dir.clone();

    let mut map = HashMap::new();

    map.extend(
        ifaces
            .iter()
            .map(|iface| (iface.iface_id(), iface.name.clone())),
    );

    filename.push("RGBStd.con");
    let mut file = fs::File::create(&filename).unwrap();
    for iface in ifaces {
        writeln!(file, "{}", iface.display(&map, &ifsys)).unwrap();
    }

    let mut ifaces = vec![rgb20::iface::rgb20_base(), rgb20::iface::rgb20_renamable()];
    ifaces.extend(
        rgb20::Features::ENUMERATE
            .iter()
            .copied()
            .map(Rgb20::<Dumb>::iface),
    );

    map.extend(
        ifaces
            .iter()
            .map(|iface| (iface.iface_id(), iface.name.clone())),
    );

    filename.pop();
    filename.push("RGB20.con");
    let mut file = fs::File::create(&filename).unwrap();
    for iface in ifaces {
        writeln!(file, "{}", iface.display(&map, &ifsys)).unwrap();
    }

    Ok(())
}
