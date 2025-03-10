// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::utils::CONSOLE;

pub(crate) fn sbi_console_putchar(c: u8) {
    CONSOLE.get().write(c);
}

pub(crate) fn sbi_console_getchar() -> u8 {
    CONSOLE.get().read()
}
