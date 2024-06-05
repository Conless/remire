// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tre

use buddy_system_allocator::LockedHeap;

const USER_HEAP_SIZE: usize = 4096 * 4;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
pub static HEAP: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
}

