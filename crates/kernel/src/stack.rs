// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{config::*, trap::TrapContext};

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

pub static KERNEL_STACK: [KernelStack; APP_MAX_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; APP_MAX_NUM];

pub static USER_STACK: [UserStack; APP_MAX_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; APP_MAX_NUM];

impl KernelStack {
    /// Get the stack pointer of the kernel stack
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// Push the context to the kernel stack
    pub fn push_context(&self, ctx: TrapContext) -> usize {
        let ctx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *ctx_ptr = ctx;
        }
        ctx_ptr as usize
    }
}

impl UserStack {
    /// Get the stack pointer of the user stack
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
