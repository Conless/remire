#![no_std]
#![no_main]
#![feature(naked_functions, asm_const)]

mod clint;
mod config;
mod legacy;
mod stack;
mod trap;
mod utils;

use crate::utils::hart_id;
use crate::utils::riscv_spec::{mepc, mie, mstatus};
use utils::set_pmp;
use core::arch::asm;
use trap::{trap_return, trap_vec};

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    asm!(
        "   call {locate_stack}
            call {rust_main}
            j    {trap}
        ",
        locate_stack = sym stack::locate,
        rust_main    = sym rust_main,
        trap         = sym trap_return,
        options(noreturn),
    )
}

extern "C" fn rust_main() {
    utils::init_bss();
    utils::init_uart();
    set_pmp();
    stack::trap_stack_init();
    clint::clear();
    mstatus::update(|bits| {
        *bits &= !mstatus::MPP;
        *bits |= mstatus::MPIE | mstatus::MPP_SUPERVISOR;
    });
    mie::write(mie::MSIE | mie::MTIE);
    unsafe {
        asm!("csrw mideleg,    {}", in(reg) !0);
        asm!("csrw medeleg,    {}", in(reg) !0);
        asm!("csrw mcounteren, {}", in(reg) !0);
        use riscv::register::{medeleg, mtvec};
        medeleg::clear_supervisor_env_call();
        medeleg::clear_machine_env_call();
        mtvec::write(trap_vec as _, mtvec::TrapMode::Vectored);
    }
}