// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::sync::atomic::AtomicPtr;
use lazy_static::lazy_static;

use crate::uart::Uart16550;

// I don't think this way of console output is elegant, and I plan to replace it with VirtIOConsole later.
// Question: how to output something in kernel state?
lazy_static! {
    pub static ref CONSOLE: Uart16550<AtomicPtr<u8>> = unsafe { Uart16550::new(0x10_000_000) }; // Device for console I/O at base address 0x10000000
}

pub fn sbi_console_putchar(c: u8) -> i32 {
    match CONSOLE.send(c) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub fn sbi_console_getchar() -> i32 {
    match CONSOLE.recv() {
        Ok(c) => c as i32,
        Err(_) => -1,
    }
}
