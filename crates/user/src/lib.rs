// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use syscall::*;

#[macro_use]
pub mod console; // For export the macros
mod lang;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main());
    unreachable!()
}

/// A fake main function
///
/// This function is created for _start to find it. When linking the code with applications, since the linkage is marked as weak, the linker will use the main function in the user applications.
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("The symbol main is not found!");
}

/// Wrapper for sys_write
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

/// Wrapper for sys_exit
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize { sys_yield() }

pub fn get_time() -> isize {
    sys_get_time()
}

pub fn sbrk(size: i32) -> isize {
    sys_sbrk(size)
}