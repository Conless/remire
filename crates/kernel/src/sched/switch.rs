// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

core::arch::global_asm!(include_str!("switch.S"));

use super::thread_info::ThreadInfo;

// Wrapper for the assembly function `__switch`
extern "C" {
    pub fn __switch(current_thread: *mut ThreadInfo, next_thread: *const ThreadInfo);
}
