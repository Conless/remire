// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]

use core::{alloc::{GlobalAlloc, Layout}, borrow::BorrowMut};

use spin::Mutex;

use self::buddy::BuddyAllocator;

mod buddy;
mod avl;

pub struct LockedAllocator(Mutex<BuddyAllocator>);

impl LockedAllocator {
    pub const fn empty() -> Self {
        LockedAllocator(Mutex::new(BuddyAllocator::empty()))
    }

    pub unsafe fn init(&self, start: usize, end: usize) {
        unsafe {
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
