// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod context;
mod timer;

pub use context::TrapContext;
pub use timer::{get_time, get_time_ms};

use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::utvec::TrapMode;
use riscv::register::{scause, sie, stval, stvec};

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::sched::proc::{current_pid, current_trap_ctx, current_user_token};
use crate::sched::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::syscall::syscall;
use crate::{log, println};
use core::arch::{asm, global_asm};

use self::timer::set_next_interrupt;

global_asm!(include_str!("trap.S"));

/// Initialize trap handling
///
/// Set the trap entry to `__alltraps` in `trap.S`
pub fn init() {
    set_kernel_trap_entry()
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

fn resolve_message() {
    crate::services::reply_services()
}

/// Trap handler
///
/// This function is the entry of handler of traps from user mode to supervisor mode.
#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            if current_pid() != 0 {
                suspend_current_and_run_next();
            }
        }
        Trap::Exception(Exception::UserEnvCall) => {
            // log!(
            //     "[kernel] receive syscall {:?}.",
            //     current_trap_ctx().regs[17]
            // );
            let mut ctx = current_trap_ctx();
            ctx.pc += 4;
            let result = syscall(ctx.regs[17], [ctx.regs[10], ctx.regs[11], ctx.regs[12]]) as usize;
            ctx = current_trap_ctx();
            ctx.regs[10] = result;
            // log!(
            //     "[kernel] return from syscall {:?}, result = {}",
            //     ctx.regs[17], ctx.regs[10]
            // );
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next(-3);
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return()
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    resolve_message();
    set_next_interrupt();
    let trap_ctx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    // log!("[kernel] return to user mode, satp = {:#x}", user_satp,);
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_ctx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}
