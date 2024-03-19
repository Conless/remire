// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![no_main]
// #![deny(warnings)]
#![feature(panic_info_message)]

mod panic;

#[no_mangle]
extern "C" fn _start() -> ! {
    // This is the entry point of the kernel
    loop {}
}

/// This function is made only to make `cargo test` and analyzer happy
#[cfg(test)]
fn main() {}
