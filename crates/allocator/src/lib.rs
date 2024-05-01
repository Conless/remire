// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]

use core::{alloc::{GlobalAlloc, Layout}, borrow::BorrowMut};

use spin::Mutex;
pub use stack::StackAllocator;

use self::buddy::BuddyAllocator;

mod buddy;
mod avl;
mod stack;

pub struct LockedBuddyAllocator(Mutex<BuddyAllocator>);

impl LockedBuddyAllocator {
    pub const fn empty() -> Self {
        Self(Mutex::new(BuddyAllocator::empty()))
    }

    /// Initializes the locked buddy allocator with the specified memory range.
    ///
    /// # Safety
    /// This function is unsafe because it operates on raw pointers and can cause undefined behavior if used incorrectly.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting address of the memory range.
    /// * `end` - The ending address of the memory range.
    pub unsafe fn init(&self, start: usize, end: usize) {
        unsafe {
            self.0.lock().borrow_mut().add_segment(start, end);
        }
    }
}

unsafe impl GlobalAlloc for LockedBuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().borrow_mut().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().borrow_mut().dealloc(ptr, layout);
    }
}
