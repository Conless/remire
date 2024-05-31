// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![no_main]
// #![deny(warnings)]
#![feature(naked_functions, asm_const, panic_info_message, arbitrary_self_types)]

use core::arch::{asm, global_asm};

extern crate alloc;

use mm::{activate_kernel_space, init_frame_allocator};
use alloc::boxed::Box;
use drivers::init_device;
use allocator::init_heap_allocator;
use task::pid::init_pid_allocator;

mod allocator;
mod lang;
mod sbi;
mod config;
mod console;
mod trap;
mod stack;
mod syscall;
mod sync;
mod task;
mod mm;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
extern "C" fn rust_init() -> ! {
    init_heap_allocator();
    init_frame_allocator();
    init_pid_allocator(0, 127);
    activate_kernel_space();
    rust_main()
}

fn rust_main() -> ! {
    log!("[kernel] Hello, World!");
    trap::init();
    task::list_apps();
    trap::enable_timer_interrupt();
    task::add_all_tasks();
    task::run_first_task()
}

/// This function is made only to make `cargo test` and analyzer happy
#[cfg(test)]
fn main() {}
