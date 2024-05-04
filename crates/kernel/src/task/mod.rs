// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod info;
pub mod loader;
pub mod manager;
pub mod pid;
mod proc;
mod switch;

use alloc::sync::Arc;
use lazy_static::lazy_static;
use switch::__switch;
use crate::{println, sbi::shutdown, sync::UPSafeCell};
pub use info::context::TaskContext;
use info::task_struct::{TaskStatus, TaskStruct};
use loader::{get_app_data, get_app_data_by_name, get_num_app};
pub use loader::load_apps;
use manager::{add_task, pop_task};
use proc::Processor;

use crate::trap::TrapContext;

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = 
        unsafe { UPSafeCell::new(Processor::default()) };
    pub static ref INITPROC: Arc<TaskStruct> = Arc::new(
        TaskStruct::new(get_app_data_by_name("initproc").unwrap())
    );
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

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.get_user_token()
}
///Get the mutable reference to trap context of current task
pub fn current_trap_ctx() -> &'static mut TrapContext {
    current_task().unwrap().inner.borrow_mut().ctx_ppn.get_mut()
}

pub fn suspend_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner.borrow_mut();
    let task_ctx_ptr = &mut task_inner.ctx as *mut TaskContext;
    task_inner.status = TaskStatus::Ready;
    drop(task_inner);

    add_task(task);
    schedule(task_ctx_ptr);
}

pub fn run_first_task() -> ! {
    run_tasks()
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let task = take_current_task().unwrap();
    let pid = task.pid.0;

    if pid == 0 {
        println!(
            "[kernel] Idle process exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            shutdown(true)
        } else {
            shutdown(false)
        }
    }
    
    let mut task_inner = task.inner.borrow_mut();
    task_inner.status = TaskStatus::Zombie(exit_code);
    
    let mut init_task_inner = INITPROC.inner.borrow_mut();
    for child in task_inner.children.iter() {
        child.inner.borrow_mut().parent = Some(Arc::downgrade(&INITPROC));
        init_task_inner.children.push(child.clone());
    }
    
    task_inner.children.clear();
    task_inner.mm.recycle();
    drop(task_inner);
    drop(task);

    let mut empty_ctx = TaskContext::default();
    schedule(&mut empty_ctx as *mut TaskContext)
}

pub fn add_all_tasks() {
    add_task(INITPROC.clone())
}

pub fn change_program_brk(size: i32) -> Option<usize> {
    if let Some(task) = PROCESSOR.borrow_mut().current() {
        let mut task_inner = task.inner.borrow_mut();
        task_inner.mm.change_brk(size)
    } else {
        None
    }
}
