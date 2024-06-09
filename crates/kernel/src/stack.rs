// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use allocator::StackAllocator;
use ksync::UPSafeCell;
use lazy_static::lazy_static;

use crate::mm::{MapPermission, VirtAddr};
use crate::mm::KERNEL_SPACE;
use crate::log;
use crate::config::*;

#[derive(Debug)]
pub struct KernelStack {
    top: usize,
    bottom: usize,
}

lazy_static! {
    static ref KERNEL_STACK_ALLOCATOR: UPSafeCell<StackAllocator> = unsafe {
        UPSafeCell::new(StackAllocator::new(0, KERNEL_STACK_NUM))
    };
}

fn get_kernel_stack_addr(id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (top, bottom)
}

impl KernelStack {
    pub fn new_process() -> Self {
        let id = KERNEL_STACK_ALLOCATOR.borrow_mut().alloc().unwrap();
        let (top, bottom) = get_kernel_stack_addr(id);
        log!("[kernel] mapping kernel stack [{:#x}, {:#x})", bottom, top);
        KERNEL_SPACE.borrow_mut().insert(
            bottom.into(),
            top.into(),
            MapPermission::R | MapPermission::W,
        );
        Self { top, bottom }
    }
    
    pub fn get_top(&self) -> usize {
        self.top
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        if self.top == 0 && self.bottom == 0 {
            return;
        }
        log!(
            "[kernel] unmapping kernel stack [{:#x}, {:#x})",
            self.bottom, self.top);
        let start_va: VirtAddr = self.bottom.into();
        KERNEL_SPACE.borrow_mut().remove(start_va.into());
    }
}