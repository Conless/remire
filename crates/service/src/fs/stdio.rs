// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use core::fmt::{Arguments, Write};

use crate::{drivers::uart::{console_getchar, CONSOLE}, yield_};

use super::File;

pub struct Stdin;
pub struct Stdout;

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        // assert_eq!(user_buf.len(), 1);
        // let mut c: usize;
        // loop {
        //     c = console_getchar() as usize;
        //     if c == 0 {
        //         yield_();
        //         continue;
        //     } else {
        //         break;
        //     }
        // }
        // let ch = c as u8;
        // unsafe {
        //     user_buf.buffers[0].as_mut_ptr().write_volatile(ch);
        // }
        1
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl Stdout {
    pub fn putchar(&self, c: char) {
        CONSOLE.get().write(c as u8)
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

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        for buffer in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        user_buf.len()
    }
}
