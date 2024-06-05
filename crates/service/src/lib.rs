// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod console;
mod allocator;
mod config;
mod lang_items;
pub mod syscall;
pub mod task;
pub mod msg;

use allocator::init_heap;
use msg::init_msg_handler;
use syscall::*;


#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(send_va: usize, recv_va: usize, start_id: usize, end_id: usize) -> ! {
    init_heap();
    init_msg_handler(send_va, recv_va, start_id, end_id);
    exit(main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

