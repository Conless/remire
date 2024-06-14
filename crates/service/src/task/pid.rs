// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::vec::Vec;
use allocator::StackAllocator;
use lazy_static::lazy_static;

use ksync::UPSafeCell;

use crate::config::MAX_PID;

lazy_static! {
    pub static ref PID_ALLOCATOR: UPSafeCell<StackAllocator> = 
      unsafe { UPSafeCell::new(StackAllocator::new(1, MAX_PID)) };
}

pub fn init_pid_allocator(min: usize, max: usize) {
    PID_ALLOCATOR.borrow_mut().init(min, max);
}

pub struct PIDGuard(pub usize);

impl Drop for PIDGuard {
  fn drop(&mut self) {
      PID_ALLOCATOR.borrow_mut().dealloc(self.0);
  }
}

pub fn alloc_pid() -> Option<PIDGuard> {
  let pid = PID_ALLOCATOR.borrow_mut().alloc();
  pid.map(PIDGuard)
}
