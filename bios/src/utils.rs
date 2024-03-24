// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::fmt::{Arguments, Write};

use crate::legacy::console::*;

struct Stdin;
struct Stdout;

impl Stdout {
    pub fn putchar(&self, c: char) {
        sbi_console_putchar(c as u8);
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
        $crate::utils::Stdout.print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::utils::Stdout.print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}

