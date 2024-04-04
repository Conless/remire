// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// The implementation of this file has referenced rustsbi-qemu.

use crate::{
    constants::*,
    trap::fast_handler,
    utils::{hart_id, HsmCell, LocalHsmCell, RemoteHsmCell},
    Supervisor,
};
use core::{cell::UnsafeCell, mem::forget, ptr::NonNull};
use fast_trap::{FlowContext, FreeTrapStack};

/// Stack for a single hardware thread
#[repr(C, align(128))]
struct Stack([u8; MACHINE_STACK_SIZE]);

impl Stack {
    /// Initial value of the stack
    const ZERO: Self = Self([0; MACHINE_STACK_SIZE]);

    /// Get the hart trap context from current stack
    ///
    /// The trap stack is divided into three parts, which are, from top to bottom:
    /// - Trap context
    /// - Stack
    /// - Fast trap path (as defined in [fast-trap](https://crates.io/crates/fast-trap))
    #[inline]
    fn hart_context(&mut self) -> &mut TrapContext {
        unsafe { &mut *self.0.as_mut_ptr().cast() }
    }

    fn load_as_stack(&'static mut self) {
        let hart = self.hart_context();
        let context_ptr = hart.context_ptr();
        hart.init();
        let range = self.0.as_ptr_range();
        forget(
            FreeTrapStack::new(
                range.start as usize..range.end as usize,
                |_| {},
                context_ptr,
                fast_handler,
            )
            .unwrap()
            .load(),
        );
    }
}

/// Trap context of current hardware thread。
struct TrapContext {
    pub trap: FlowContext,
    pub hsm: HsmCell<Supervisor>,
}

impl TrapContext {
    #[inline]
    fn init(&mut self) {
        self.hsm = HsmCell::new();
    }

    #[inline]
    fn context_ptr(&mut self) -> NonNull<FlowContext> {
        unsafe { NonNull::new_unchecked(&mut self.trap) }
    }
}

/// Stack of M-mode trap
#[link_section = ".bss.uninit"]
static mut MACHINE_STACK: [Stack; HART_MAX] = [Stack::ZERO; HART_MAX];

/// Locate the stack of current hardware thread
///
/// This function is called by `start` function in `src/main.rs`
///
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
            call t1, {move_stack}
            ret
        ",
        per_hart_stack_size = const MACHINE_STACK_SIZE,
        stack               =   sym MACHINE_STACK,
        move_stack          =   sym fast_trap::reuse_stack_for_trap,
        options(noreturn),
    )
}

/// Initiate the trap stack
pub(crate) fn prepare_for_trap() {
    unsafe { MACHINE_STACK.get_unchecked_mut(hart_id()).load_as_stack() };
}

pub(crate) fn local_hsm() -> LocalHsmCell<'static, Supervisor> {
    unsafe {
        MACHINE_STACK
            .get_unchecked_mut(hart_id())
            .hart_context()
            .hsm
            .local()
    }
}

pub(crate) fn local_remote_hsm() -> RemoteHsmCell<'static, Supervisor> {
    unsafe {
        MACHINE_STACK
            .get_unchecked_mut(hart_id())
            .hart_context()
            .hsm
            .remote()
    }
}