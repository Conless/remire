// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::write;
use core::fmt::{self, Write};

struct Stdout;
struct Stderr;

const STDOUT: usize = 1;
const STDERR: usize = 2;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

impl Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDERR, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

pub fn eprint(args: fmt::Arguments) {
    Stderr.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! eprint {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::eprint(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! eprintln {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::eprint(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
