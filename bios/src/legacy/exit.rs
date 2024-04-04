// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use exit_values::*;

pub(crate) mod exit_values {
    pub const QEMU_EXIT_ADDR: usize = 0x100000;
    pub const QEMU_EXIT_SUCC: u32 = 0x5555;
    pub const QEMU_EXIT_FAIL: u32 = 0x3333;
    pub const QEMU_EXIT_RESET: u32 = 0x7777;
}

pub(crate) fn sbi_shutdown() -> ! {
    unsafe {
        asm!(
            "sw {0}, 0({1})",
            in(reg) QEMU_EXIT_SUCC,
            in(reg) QEMU_EXIT_ADDR
        );
    }
    unreachable!()
}
