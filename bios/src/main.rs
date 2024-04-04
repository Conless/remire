// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! This file contains the entry point of the bios. It also provides the RISC-V SBI (Supervisor Binary Interface) implementation for the operating system running on qemu-system-riscv64.
//! The implementation of this file is based on a minor version of RISC-V SBI spec 2.0, and references [rustsbi-qemu](https://github.com/rustsbi/rustsbi-qemu)

#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]
// #![deny(warnings, missing_docs)]

mod lang;
mod stack;
mod trap;
mod utils;

mod legacy; // Chapter 5

use constants::*;
use sbi_spec::hsm::hart_state;
use core::{arch::asm, sync::atomic::{AtomicUsize, Ordering}};
use riscv::{
    asm,
    register::{medeleg, mtvec},
};

#[derive(Debug)]
struct Supervisor {
    start_addr: usize,
    opaque: usize,
}

/// Constants for the bios
pub(crate) mod constants {
    pub const MACHINE_STACK_SIZE: usize = 4 * 1024;
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
        trap = sym trap::trap_handler,
        options(noreturn)
    )
}

static mut ATOMIC_TEST: AtomicUsize = AtomicUsize::new(0);

extern "C" fn rust_main(hartid: usize, opaque: usize) {
    legacy::uart::console_init();
    stack::prepare_for_trap();
    println!("Hello world from BIOS!");
    let board_info = utils::parse(opaque);
    utils::set_pmp(&board_info);
    stack::local_remote_hsm().start(Supervisor {
        start_addr: OS_ENTRY_ADDR,
        opaque,
    });
    unsafe {
        asm!("csrw mideleg,    {}", in(reg) !0);
        asm!("csrw medeleg,    {}", in(reg) !0);
        asm!("csrw mcounteren, {}", in(reg) !0);
        medeleg::clear_supervisor_env_call();
        medeleg::clear_machine_env_call();
        mtvec::write(trap::trap_handler as usize, mtvec::TrapMode::Direct);
    }
}

