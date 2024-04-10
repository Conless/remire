// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod context;

pub use context::TrapContext;
use riscv::register::scause::{Exception, Trap};
use riscv::register::utvec::TrapMode;
use riscv::register::{scause, stval, stvec};

use crate::syscall::syscall;
use crate::{batch::run_next_app, println};
use core::arch::global_asm;

global_asm!(include_str!("trap.S"));

/// Initialize trap handling
///
/// Set the trap entry to `__alltraps` in `trap.S`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

/// Trap handler
///
/// This function is the entry of handler of traps from user mode to supervisor mode.
#[no_mangle]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.pc += 4;
            ctx.regs[10] =
                syscall(ctx.regs[17], [ctx.regs[10], ctx.regs[11], ctx.regs[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    ctx
}
