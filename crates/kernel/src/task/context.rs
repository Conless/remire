// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{stack::{KERNEL_STACK, USER_STACK}, trap::TrapContext};

use super::loader::get_app_addr;

/// Task Context
///
/// This struct is used to store the context of a task, containing the return address of the task, the stack pointer of the task, and the callee-saved registers.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn restore(sp: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp,
            s: [0; 12],
        }
    }
}
