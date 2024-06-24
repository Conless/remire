// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


use crate::trap::TrapContext;
use crate::{config::LEN_STACK_PER_HART, config::NUM_HART_MAX, hart_id};

#[link_section = ".bss.uninit"]
static mut ROOT_STACK: [Stack; NUM_HART_MAX] = [Stack::ZERO; NUM_HART_MAX];

#[naked]
pub(crate) unsafe extern "C" fn locate() {
    core::arch::asm!(
        "   la   sp, {stack}
            li   t0, {per_hart_stack_size}
            csrr t1, mhartid
            addi t1, t1,  1
         1: add  sp, sp, t0
            addi t1, t1, -1
            bnez t1, 1b
            ret
        ",
        per_hart_stack_size = const LEN_STACK_PER_HART,
        stack               =   sym ROOT_STACK,
        options(noreturn),
    )
}

#[repr(C, align(128))]
struct Stack([u8; LEN_STACK_PER_HART]);

impl Stack {
    const ZERO: Self = Self([0; LEN_STACK_PER_HART]);

    #[inline]
    fn trap_ctx_ptr(&mut self) -> &mut TrapContext {
        unsafe { &mut *self.0.as_mut_ptr().cast() }
    }
    
    fn init(&'static mut self) {
        let sp = self.0.as_ptr_range().end as usize;
        let trap_ctx = self.trap_ctx_ptr();
        trap_ctx.init(sp);
    }
}


pub fn trap_context() -> &'static mut TrapContext {
    unsafe { ROOT_STACK.get_unchecked_mut(hart_id()).trap_ctx_ptr() }
}

pub fn trap_stack_init() {
    unsafe { ROOT_STACK.get_unchecked_mut(hart_id()).init() };
}
