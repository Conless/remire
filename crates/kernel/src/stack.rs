// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::mm::{MapPermission, KERNEL_SPACE};
use crate::{config::*, trap::TrapContext};
use crate::task::pid::PIDGuard;

#[derive(Default, Debug)]
pub struct KernelStack {
    top: usize,
    bottom: usize,
}

fn get_kernel_stack_addr(pid: usize) -> (usize, usize) {
    let top = TRAMPOLINE - pid * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (top, bottom)
}

impl KernelStack {
    pub fn new(pid: &PIDGuard) -> Self {
        let pid = pid.0;
        let (top, bottom) = get_kernel_stack_addr(pid);
        KERNEL_SPACE.borrow_mut().insert(
            bottom.into(),
            top.into(),
            MapPermission::R | MapPermission::W,
        );
        KernelStack { top, bottom }
    }
    
    pub fn get_top(&self) -> usize {
        self.top
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        KERNEL_SPACE.borrow_mut().remove(self.bottom.into());
    }
}