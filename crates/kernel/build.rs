// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use std::{
    collections::BTreeMap, fs::{self, File}, path::{Path, PathBuf}
};
use std::io::Write;

struct AppCategory {
    name: String,
    apps: BTreeMap<String, PathBuf>,
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}

fn insert_app_data(name_to_bin: Vec<AppCategory>) -> Result<(), std::io::Error> {
    let mut f = File::create(project_root().join("crates/kernel/src/link_app.S")).unwrap();

    for category in name_to_bin {
        writeln!(
            f,
            r#"
    .align 3
    .section .data
    .global _num_{0}
_num_{0}:
    .quad {1}"#,
            category.name,
            category.apps.len()
        )?;
        
        for i in 0..category.apps.len() {
            writeln!(f, r#"    .quad {}_{}_start"#, category.name, i)?;
        }
        writeln!(f, r#"    .quad {}_{}_end"#, category.name, category.apps.len() - 1)?;

        writeln!(
            f,
            r#"
    .global _{0}_names
_{0}_names:"#,
            category.name
        )?;
        for (app_name, _) in category.apps.iter() {
            writeln!(f, r#"    .string "{}""#, app_name)?;
        }
        for (idx, (app_name, bin)) in category.apps.iter().enumerate() {
            println!("{}_{}: {}", category.name, idx, app_name);
            writeln!(
                f,
                r#"
    .section .data
    .global {2}_{0}_start
    .global {2}_{0}_end
    .align 3
{2}_{0}_start:
    .incbin "{1}"
{2}_{0}_end:"#,
                idx,
                bin.to_str().unwrap(),
                category.name
            )?;
        }
    }
    Ok(())
}


const APPS_CATEGORIES: [&str; 2] = ["app", "service"];

fn main() {
    println!("cargo:rustc-link-arg=-Tcrates/kernel/src/linker.ld");
    println!("cargo:rustc-env=RUSTFLAGS=-Cforce-frame-pointers=yes");
    let mut name_to_bin = Vec::new();
    for category in &APPS_CATEGORIES {
        println!("cargo:rerun-if-changed=../{}", category);
        let mut app_name_to_bin = BTreeMap::new();
        let target = "riscv64gc-unknown-none-elf";
        let mode = "debug";
        let app_dir = project_root().join("crates").join(category).join("src/bin");
        let target_dir = project_root().join("target").join(target).join(mode);
        let apps: Vec<_> = fs::read_dir(app_dir)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        for app in &apps {
            let app_name = app.file_stem().unwrap().to_str().unwrap();
            let elf = target_dir.join(app_name);
            app_name_to_bin.insert(app_name.to_string(), elf.clone());
        }
        name_to_bin.push(AppCategory {
            name: category.to_string(),
            apps: app_name_to_bin,
        });
    }
    insert_app_data(name_to_bin).unwrap();
}
