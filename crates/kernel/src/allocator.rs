// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use allocator::LockedAllocator;

use crate::config::KERNEL_HEAP_SIZE;

#[global_allocator]
static HEAP_ALLOCATOR: LockedAllocator = LockedAllocator::empty();

static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap_allocator() {
    unsafe {
        let start = KERNEL_HEAP_SPACE.as_ptr() as usize;
        let end = start + KERNEL_HEAP_SPACE.len();
        HEAP_ALLOCATOR.init(start, end);
    }
}
