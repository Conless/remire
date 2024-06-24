// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub status: Sstatus,
    pub pc: usize,
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.regs[2] = sp;
    }
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut ctx = Self {
            regs: [0; 32],
            status: sstatus,
            pc: entry,
            kernel_satp: satp,
            kernel_sp,
            trap_handler,
        };
        ctx.set_sp(sp);
        ctx
    }
}
