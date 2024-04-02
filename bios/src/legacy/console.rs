// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::uart::CONSOLE;

pub(crate) fn sbi_console_putchar(c: u8) {
    unsafe {
        CONSOLE.get().write(c);
    }
}

pub(crate) fn sbi_console_getchar() -> u8 {
    unsafe {
        CONSOLE.get().read()
    }
}
