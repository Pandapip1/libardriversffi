// SPDX-FileCopyrightText: Copyright 2024 Gavin John
// SPDX-License-Identifier: GPL-3.0-or-later

use cbindgen::{Config, Language};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: Config = Default::default();
    config.language = Language::C;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/include/ar_drivers.h");
}
