// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::{sync::UPSafeCell, trap::TrapContext};

use super::{info::task_struct::{TaskStatus, TaskStruct}, manager::pop_task, switch::__switch, TaskContext};

#[derive(Default)]
pub struct Processor {
    current: Option<Arc<TaskStruct>>,
    idle_task: TaskContext,
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

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = 
        unsafe { UPSafeCell::new(Processor::default()) };
}

pub fn take_current_task() -> Option<Arc<TaskStruct>> {
    PROCESSOR.borrow_mut().take_current()
}

pub fn current_task() -> Option<Arc<TaskStruct>> {
    PROCESSOR.borrow_mut().current()
}

pub fn run_tasks() -> ! {
  loop {
      let mut processor = PROCESSOR.borrow_mut();
      if let Some(task) = pop_task() {
          let idle_task_ctx_ptr = processor.task_ctx_ptr();
          let mut task_inner = task.inner.borrow_mut();
          let next_task_ctx_ptr = &task_inner.ctx as *const TaskContext;
          task_inner.status = TaskStatus::Running;
          drop(task_inner);

          processor.current = Some(task);
          drop(processor);
          unsafe {
              __switch(idle_task_ctx_ptr, next_task_ctx_ptr);
          }
      }
  }
}

pub fn schedule(switched_task_ctx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.borrow_mut();
    let idle_task_ctx_ptr = processor.task_ctx_ptr();
    drop(processor);
    unsafe {
        __switch(
            switched_task_ctx_ptr,
            idle_task_ctx_ptr,
        );
    }
}
