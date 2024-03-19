// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::{env, fs, path::PathBuf};

/// Build script for the kernel
/// 
/// I consider it ugly to put the linker script in the global config.toml, so I replace it with this.
/// The implementation of this script is based on [recore](https://github.com/Celve/recore/blob/main/kernel/build.rs).
fn main() {
  let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
  fs::write(ld, LINKER).unwrap();
  println!("cargo:rustc-link-arg=-T{}", ld.display());
}

/// Linker file of the RISC-V executable running on qemu, started at position 0x80000000.
/// Copied from [rcore](https://github.com/rcore-os/rCore-Tutorial-v3/blob/main/os/src/linker-qemu.ld)
const LINKER: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}";
