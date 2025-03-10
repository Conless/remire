// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub const KERNEL_STACK_SIZE: usize = 4096 * 8;
pub const KERNEL_STACK_NUM: usize = 256;
pub const USER_STACK_SIZE: usize = 4096 * 4;

pub const APP_MAX_NUM: usize = 16;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const KERNEL_HEAP_SIZE: usize = 0x300000;

pub const MEMORY_END: usize = 0x88000000;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
pub const SERVICE_SEND_PORT: usize = TRAMPOLINE - PAGE_SIZE * 4;
pub const SERVICE_RECV_PORT: usize = TRAMPOLINE - PAGE_SIZE * 7;

pub const CLOCK_FREQ: usize = 12500000;
pub const MMIO: &[(usize, usize)] = &[
    (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC  in virt machine
];

pub const LOG: bool = false;
