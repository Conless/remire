// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! This file contains the entry point of the bios. It also provides the RISC-V SBI (Supervisor Binary Interface) implementation for the operating system running on qemu-system-riscv64.
//! The implementation of this file is based on a minor version of RISC-V SBI spec 2.0, and references [rustsbi-qemu](https://github.com/rustsbi/rustsbi-qemu)

#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
// #![deny(warnings)]

mod lang;
mod stack;
mod trap;
mod utils;

mod legacy; // Chapter 5

use constants::*;
use core::arch::asm;
use riscv::{asm, register::mtvec};

/// Constants for the bios
pub(crate) mod constants {
    pub const BIOS_STACK_SIZE: usize = 4 * 1024;
    pub const OS_ENTRY_ADDR: usize = 0x8020_0000;
    pub const OS_STACK_SIZE: usize = 16 * 1024;
    pub const HART_MAX: usize = 1;
}

/// The entry point of the bios
///
/// This function inits stack of the bios, and jumps to the `rust_main` of bios.
/// Note that this function has to be marked as `naked` to avoid the prologue and epilogue, otherwise it may not be placed at the start address of qemu
#[no_mangle]
#[link_section = ".text.entry"]
#[naked]
unsafe extern "C" fn _start() -> ! {
    asm!(
        "   call {stack_init}
            call {rust_main}
            j {trap}
        ",
        stack_init = sym stack::locate,
        rust_main = sym rust_main,
        trap = sym trap::mtrap_handler,
        options(noreturn)
    )
}

extern "C" fn rust_main() {
    legacy::console::console_init();
    unsafe {
        mtvec::write(trap::mtrap_handler as usize, mtvec::TrapMode::Direct);
    }
    // Jump to os entry
    unsafe {
        asm!(
            "   li t0, {os_entry}
                jr t0
            ",
            os_entry = const OS_ENTRY_ADDR
        )
    }
}
