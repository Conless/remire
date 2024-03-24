// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use crate::constants::*;

#[link_section = ".bss"]
static mut BIOS_STACK: [u8; BIOS_STACK_SIZE] = [0; BIOS_STACK_SIZE];

#[naked]
pub(crate) unsafe extern "C" fn locate() {
    asm!(
        "   la sp, {stack}
            ret
        ",
        stack = sym BIOS_STACK,
        options(noreturn)
    )
}