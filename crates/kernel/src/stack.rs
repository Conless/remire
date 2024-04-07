// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{config::*, trap::TrapContext};

#[repr(align(4096))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
pub struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

pub static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

pub static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    /// Get the stack pointer of the kernel stack
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// Push the context to the kernel stack
    pub fn push_context(&self, ctx: TrapContext) -> &'static mut TrapContext {
        let ctx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ctx_ptr = ctx;
        }
        unsafe { ctx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    /// Get the stack pointer of the user stack
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
