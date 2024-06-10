// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
pub const SERVICE_SEND_PORT: usize = TRAMPOLINE - PAGE_SIZE * 3;
pub const SERVICE_RECV_PORT: usize = TRAMPOLINE - PAGE_SIZE * 5;

pub const LOG: bool = false;

pub const MAX_PID: usize = 255;