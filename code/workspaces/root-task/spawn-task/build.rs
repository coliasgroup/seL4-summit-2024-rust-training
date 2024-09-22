//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use std::env;
use std::fs::File;
use std::path::PathBuf;

const CHILD_VAR_ENV: &str = "CHILD_ELF";

fn main() {
    if env::var(CHILD_VAR_ENV).is_err() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let out_path = PathBuf::from(&out_dir).join("child.elf");
        File::create(&out_path).unwrap();
        println!("cargo:rustc-env=CHILD_ELF={}", out_path.display());
    }

    println!("cargo::rerun-if-env-changed={CHILD_VAR_ENV}");
}
