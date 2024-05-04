// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::{sync::UPSafeCell, trap::TrapContext};

use super::{info::task_struct::{TaskStatus, TaskStruct}, manager::pop_task, switch::__switch, TaskContext};

#[derive(Default)]
pub struct Processor {
    pub current: Option<Arc<TaskStruct>>,
    pub idle_task: TaskContext,
}

impl Processor {
    pub fn take_current(&mut self) -> Option<Arc<TaskStruct>> {
        self.current.take()
    }
    
    pub fn current(&self) -> Option<Arc<TaskStruct>> {
        self.current.clone()
    }
    
    pub fn task_ctx_ptr(&mut self) -> *mut TaskContext {
      &mut self.idle_task as *mut TaskContext
    }
}
