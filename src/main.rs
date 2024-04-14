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

use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use ifaces::{rgb20, rgb21, rgb25, IfaceWrapper, Rgb20, Rgb21, Rgb25};
use rgbstd::containers::{FileContent, Kit};
use rgbstd::stl::StandardTypes;
use strict_types::StlFormat;

fn main() -> io::Result<()> {
    let mut kit = Kit::default();
    for features in rgb20::Features::ENUMERATE {
        let iface = Rgb20::iface(*features);
        let types = StandardTypes::new()
            .type_system()
            .extract(iface.types())
            .unwrap();
        kit.ifaces.push(iface).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGB20.rgb")?;
    kit.save_armored("interfaces/RGB20.rgba")?;

    let mut kit = Kit::default();
    for features in rgb21::Features::ENUMERATE {
        let iface = Rgb21::iface(*features);
        let types = StandardTypes::with(Rgb21::stl())
            .type_system()
            .extract(iface.types())
            .unwrap();
        kit.ifaces.push(iface).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGB21.rgb")?;
    kit.save_armored("interfaces/RGB21.rgba")?;

    let mut kit = Kit::default();
    for features in rgb25::Features::ENUMERATE {
        let iface = Rgb25::iface(*features);
        let types = StandardTypes::new()
            .type_system()
            .extract(iface.types())
            .unwrap();
        kit.ifaces.push(iface).unwrap();
        kit.types.extend(types).unwrap();
    }
    kit.save_file("interfaces/RGB25.rgb")?;
    kit.save_armored("interfaces/RGB25.rgba")?;

    let dir = PathBuf::from_str("interfaces").unwrap();
    let stl = Rgb21::stl();
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

    Ok(())
}
