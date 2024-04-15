// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use syscall::{sys_exit, sys_write};

#[macro_use]
pub mod console; // For export the macros
mod lang;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss(); // Clear the .bss section as what we did in the bootloader
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

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
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
