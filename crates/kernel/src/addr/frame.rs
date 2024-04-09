// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{addr::address::PhysAddr, config::MEMORY_END, sync::UPSafeCell};

use super::{address::PhysPageNum, stack_allocator::StackFrameAllocator};

/// Interface of a frame allocator
pub trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

// We don't use dyn FrameAllocator because lazy_static require Sized, so why should we design this trait?
lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<StackFrameAllocator> =
        unsafe { UPSafeCell::new(StackFrameAllocator::new()) };
}

/// Initialize the frame allocator
pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.borrow_mut().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    );
}

/// Allocate a frame
pub fn frame_alloc() -> Option<FrameGuard> {
    FRAME_ALLOCATOR
        .borrow_mut()
        .alloc()
        .map(|ppn| FrameGuard::new(ppn))
}

/// Deallocate a frame
fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.borrow_mut().dealloc(ppn);
}

pub struct FrameGuard {
    pub ppn: PhysPageNum,
}

impl FrameGuard {
    /// Create a new frame guard and clear the frame
    pub fn new(ppn: PhysPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}
