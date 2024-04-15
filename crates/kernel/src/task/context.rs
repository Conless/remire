// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::trap::trap_return;

/// Task Context
/// 
/// This struct is used to store the context of a task, containing the return address of the task, the stack pointer of the task, and the callee-saved registers.
#[repr(C)]
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

    pub fn trap_return(sp: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp,
            s: [0; 12],
        }
    }
}
