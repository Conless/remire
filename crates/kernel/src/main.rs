// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![no_main]
// #![deny(warnings)]
#![feature(panic_info_message)]
#![feature(naked_functions)]

use core::arch::asm;

use drivers::{init_device, print, println};

mod lang;

const BOOTLOADER_STACK_SIZE: usize = 4096;

#[link_section = ".bss"]
static mut BOOTLOADER_STACK: [u8; BOOTLOADER_STACK_SIZE] = [0; BOOTLOADER_STACK_SIZE];

#[no_mangle]
#[naked]
#[link_section = ".text.entry"]
/// Start point of the kernel
///
/// Use it to override the default _start by rust compiler.
/// Note that this function has to be marked as `naked` to avoid the prologue and epilogue, otherwise it may not be placed at the start address of qemu
unsafe extern "C" fn _start() -> ! {
    // This is the entry point of the kernel
    asm!(
        "la sp, {bootloader_stack}",
        "j {rust_init}",
        bootloader_stack = sym BOOTLOADER_STACK,
        rust_init = sym rust_init,
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn rust_init() -> ! {
    init_device();
    rust_main()
}

fn rust_main() -> ! {
    println!("Hello, world!");
    println!("This is the {} message", 1);

    loop {}
}

/// This function is made only to make `cargo test` and analyzer happy
#[cfg(test)]
fn main() {}
