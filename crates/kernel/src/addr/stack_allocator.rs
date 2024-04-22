// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::vec::Vec;

use super::{address::PhysPageNum, frame::FrameAllocator};

/// A simple frame allocator that allocates frames from a recycled stack
pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    /// Initialize the stack frame allocator with a range of frames
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
    }
}

impl FrameAllocator for StackFrameAllocator {
    /// Create a new stack frame allocator
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    /// Allocate a frame
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            // There is a recycled frame
            Some(ppn.into())
        } else if self.current == self.end {
            // No more frames
            None
        } else {
            // Get one from the current frame
            self.current += 1;
            Some((self.current - 1).into())
        }
    }

    /// Deallocate a frame
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().any(|v| *v == ppn) {
            panic!("Frame ppn={:#x} is invalid!", ppn);
        }
        self.recycled.push(ppn);
    }
}
