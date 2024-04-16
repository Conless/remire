// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

core::arch::global_asm!(include_str!("switch.S"));

use super::TaskContext;

// Wrapper for the assembly function `__switch`
extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
