// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

extern crate alloc;

use alloc::vec::Vec;

/// Implementation of the stack allocator with recycled frames
#[derive(Default)]
pub struct StackAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackAllocator {
    /// Initialize the stack frame allocator with a range of frames
    pub fn init(&mut self, l: usize, r: usize) {
        self.current = l;
        self.end = r;
    }

    /// Allocate a frame
    pub fn alloc(&mut self) -> Option<usize> {
        if let Some(ppn) = self.recycled.pop() {
            // There is a recycled frame
            Some(ppn)
        } else if self.current == self.end {
            // No more frames
            None
        } else {
            // Get one from the current frame
            self.current += 1;
            Some(self.current - 1)
        }
    }

    /// Deallocate a frame
    pub fn dealloc(&mut self, ppn: usize) {
        if ppn >= self.current || self.recycled.iter().any(|v| *v == ppn) {
            panic!("Frame ppn={:#x} is invalid!", ppn);
        }
        self.recycled.push(ppn);
    }
}
