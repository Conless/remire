// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::{alloc::{GlobalAlloc, Layout}, borrow::BorrowMut};

use spin::Mutex;

use crate::config::KERNEL_HEAP_SIZE;

use self::buddy::BuddyAllocator;

mod buddy;
mod avl;

static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub struct LockedAllocator(Mutex<BuddyAllocator>);

impl LockedAllocator {
    pub const fn empty() -> Self {
        LockedAllocator(Mutex::new(BuddyAllocator::empty()))
    }

    pub fn init(&self) {
        unsafe {
            let start = KERNEL_HEAP_SPACE.as_ptr() as usize;
            let end = start + KERNEL_HEAP_SPACE.len();
            self.0.lock().borrow_mut().add_segment(start, end);
        }
    }
}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().borrow_mut().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().borrow_mut().dealloc(ptr, layout);
    }
}

#[global_allocator]
static HEAP_ALLOCATOR: LockedAllocator = LockedAllocator::empty();

pub fn init_heap_allocator() {
    HEAP_ALLOCATOR.init();
}
