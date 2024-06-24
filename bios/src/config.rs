// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

/// The implementation of this file is basically based on rustsbi-qemu

pub const MEM_START: usize = 0x8000_0000;
pub const MEM_END: usize = 0x8800_0000;

pub const CLINT_START: usize = 0x200_0000;

pub const SUPERVISOR_ENTRY: usize = 0x8020_0000;

pub const LEN_STACK_PER_HART: usize = 16 * 1024;
pub const NUM_HART_MAX: usize = 8;
