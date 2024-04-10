// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![no_main]
// #![deny(warnings)]
#![feature(naked_functions, asm_const, panic_info_message, arbitrary_self_types)]

use core::arch::{asm, global_asm};

extern crate alloc;

use alloc::boxed::Box;
use drivers::init_device;

use crate::mem::HEAP_ALLOCATOR;

mod lang;
mod sbi;
mod config;
mod batch;
mod console;
mod trap;
mod stack;
mod syscall;
mod sync;
mod mem;
mod addr;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
extern "C" fn rust_init() -> ! {
    // init_device();
    rust_main()
}

fn rust_main() -> ! {
    HEAP_ALLOCATOR.init();
    trap::init();
    Box::new(1);
    Box::new(1);
    Box::new(1);
    Box::new(1);
    Box::new(1);
    Box::new(1);
    Box::new(1);
    Box::new(1);
    batch::run_next_app();
}

/// This function is made only to make `cargo test` and analyzer happy
#[cfg(test)]
fn main() {}
