// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod uart;

/// The `console` module contains implementation of
/// - `sbi_console_putchar` (SBI version 0.1, EID 0x01)
/// - `sbi_console_getchar` (SBI version 0.1, EID 0x02)
pub mod console;

pub(crate) fn sbi_shutdown() {
  
}
