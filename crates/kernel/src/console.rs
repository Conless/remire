// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::fmt::{Arguments, Write};

use crate::sbi::{console_getchar, console_putchar};

pub struct Stdin; // Standard input agent
pub struct Stdout; // Standard output agent

impl Stdin {
    pub fn getchar(&self) -> char {
        console_getchar() as u8 as char
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut i = 0;
        while i < buf.len() {
            buf[i] = self.getchar() as u8;
            i += 1;
        }
        i
    }
}

impl Stdout {
    pub fn putchar(&self, c: char) {
        console_putchar(c as usize)
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

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if $crate::config::LOG {
            $crate::println!($($arg)*);
        }
    }
}