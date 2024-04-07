// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub const KERNEL_STACK_SIZE: usize = 4096;
pub const USER_STACK_SIZE: usize = 4096;

pub const APP_MAX_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;