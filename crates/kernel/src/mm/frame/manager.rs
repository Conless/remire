// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


use crate::config::MEMORY_END;

use super::{FrameGuard, PhysAddr, PhysPageNum};

use ksync::UPSafeCell;
use lazy_static::lazy_static;

use allocator::StackAllocator;

/// Interface of a frame manager
pub trait FrameManager : Default {
  fn init_frame(&mut self, start: PhysPageNum, end: PhysPageNum);
  fn alloc_frame(&mut self) -> Option<PhysPageNum>;
  fn dealloc_frame(&mut self, ppn: PhysPageNum);
}

impl FrameManager for StackAllocator {
  fn init_frame(&mut self, start: PhysPageNum, end: PhysPageNum) {
      self.init(start.into(), end.into())
  }

  fn alloc_frame(&mut self) -> Option<PhysPageNum> {
      self.alloc().map(|ppn| ppn.into())
  }

  fn dealloc_frame(&mut self, ppn: PhysPageNum) {
      self.dealloc(ppn.into())
  }
}

// We don't use dyn FrameAllocator because lazy_static require Sized, so why should we design this trait?
lazy_static! {
  pub static ref FRAME_ALLOCATOR: UPSafeCell<StackAllocator> =
      unsafe { UPSafeCell::new(StackAllocator::default()) };
}

/// Initialize the frame allocator
pub fn init_frame_allocator() {
  extern "C" {
      fn ekernel();
  }
  FRAME_ALLOCATOR.borrow_mut().init_frame(
      PhysAddr::from(ekernel as usize).ceil(),
      PhysAddr::from(MEMORY_END).floor(),
  );
}

/// Allocate a frame
pub fn frame_alloc() -> Option<FrameGuard> {
  FRAME_ALLOCATOR
      .borrow_mut()
      .alloc_frame()
      .map(FrameGuard::new)
}

/// Deallocate a frame
pub fn frame_dealloc(ppn: PhysPageNum) {
  FRAME_ALLOCATOR.borrow_mut().dealloc_frame(ppn);
}
