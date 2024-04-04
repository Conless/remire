// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use fast_trap::{trap_entry, FastContext, FastResult};
use riscv::register::{satp, sstatus};

use crate::{stack::local_hsm, utils::hart_id};
use crate::utils::{mstatus, mie, mepc};

#[no_mangle]
#[naked]
pub(crate) unsafe extern "C" fn trap_handler() {
    asm!(
      ".align 2",
      ".option push",
      ".option norvc",
      "j {entry}",
      entry = sym trap_entry,
      options(noreturn),
    )
}

#[no_mangle]
pub(crate) extern "C" fn fast_handler(
    mut ctx: FastContext,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> FastResult {
  #[inline]
    fn boot(mut ctx: FastContext, start_addr: usize, opaque: usize) -> FastResult {
        unsafe {
            sstatus::clear_sie();
            satp::write(0);
        }
        ctx.regs().a[0] = hart_id();
        ctx.regs().a[1] = opaque;
        ctx.regs().pc = start_addr;
        ctx.call(2)
    }
    // boot(ctx, 0x8020_0000, 0)
    loop {
        match local_hsm().start() {
            Ok(supervisor) => {
                mstatus::update(|bits| {
                    *bits &= !mstatus::MPP;
                    *bits |= mstatus::MPIE | mstatus::MPP_SUPERVISOR;
                });
                // mie::write(mie::MSIE | mie::MTIE);
                break boot(ctx, supervisor.start_addr, supervisor.opaque);
            }
            _ => {
                panic!("Hart {} is not ready", hart_id());
            }
        }
    }
}