// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{log, mm::get_kernel_stack, trap::trap_return};

/// Task Context
///
/// This struct is used to store the context of a task, containing the return address of the task, the stack pointer of the task, and the callee-saved registers.
#[repr(C)]
#[derive(Clone, Default)]
pub struct ThreadInfo {
    ra: usize,
    sp: usize,
    s: [usize; 12],
    pub pid: usize,
    pub token: usize,
}

impl ThreadInfo {
    pub fn new(pid: usize, token: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp: get_kernel_stack(token),
            s: [0; 12],
            pid,
            token,
        }
    }
    
    pub fn get_sp(&self) -> usize {
        self.sp
    }
}

impl Drop for ThreadInfo {
    fn drop(&mut self) {
        log!("[kernel] Drop thread info: pid={}, token={:x}", self.pid, self.token);
    }
}
