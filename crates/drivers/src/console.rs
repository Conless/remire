// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::{fmt::{Arguments, Write}, sync::atomic::AtomicPtr};
use lazy_static::lazy_static;

use crate::uart::uart_16650::Uart16550;

pub struct Stdout; // Standard out agent

lazy_static! {
    pub static ref UART: Uart16550<AtomicPtr<u8>> = unsafe { Uart16550::new(0x10_000_000) }; // Device for console I/O at base address 0x10000000
}

impl Stdout {
    pub fn putchar(&self, c: char) {
        UART.send(c as u8).unwrap()
    }

    pub fn print(&mut self, args: Arguments) {
        self.write_fmt(args).unwrap();
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.chars().for_each(|c| self.putchar(c));
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::Stdout.print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::console::Stdout.print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}