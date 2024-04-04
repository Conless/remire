// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub mod uart;

/// The `console` module contains implementation of
/// - `sbi_console_putchar` (SBI version 0.1, EID 0x01)
/// - `sbi_console_getchar` (SBI version 0.1, EID 0x02)
pub mod console;

/// The `exit` module contains implementation of
/// - `sbi_shutdown` (SBI version 0.1, EID 0x08)
pub mod exit;
