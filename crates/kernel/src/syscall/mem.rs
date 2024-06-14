// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{mm::change_program_brk, sched::proc::current_pid};

pub fn sys_sbrk(size: i32) -> isize {
    if let Some(old_brk) = change_program_brk(current_pid(), size) {
        old_brk as isize
    } else {
        -1
    }
}
