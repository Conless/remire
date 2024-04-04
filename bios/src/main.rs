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

extern "C" fn rust_main() {
    legacy::uart::console_init();
    stack::prepare_for_trap();
    println!("Hello world from BIOS!");
    unsafe {
        asm!("csrw mideleg,    {}", in(reg) !0);
        asm!("csrw medeleg,    {}", in(reg) !0);
        asm!("csrw mcounteren, {}", in(reg) !0);
        medeleg::clear_supervisor_env_call();
        medeleg::clear_machine_env_call();
        mtvec::write(trap::trap_handler as usize, mtvec::TrapMode::Direct);
    }
}

fn set_pmp(board_info: &BoardInfo) {
    use riscv::register::*;
    let mem = &board_info.mem;
    unsafe {
        pmpcfg0::set_pmp(0, Range::OFF, Permission::NONE, false);
        pmpaddr0::write(0);
        // 外设
        pmpcfg0::set_pmp(1, Range::TOR, Permission::RW, false);
        pmpaddr1::write(mem.start >> 2);
        // SBI
        pmpcfg0::set_pmp(2, Range::TOR, Permission::NONE, false);
        pmpaddr2::write(SUPERVISOR_ENTRY >> 2);
        // 主存
        pmpcfg0::set_pmp(3, Range::TOR, Permission::RWX, false);
        pmpaddr3::write(mem.end >> 2);
        // 其他
        pmpcfg0::set_pmp(4, Range::TOR, Permission::RW, false);
        pmpaddr4::write(1 << (usize::BITS - 1));
    }
}
