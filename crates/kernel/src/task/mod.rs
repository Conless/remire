// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod info;
mod loader;
mod manager;
pub mod pid;
mod proc;
mod switch;

use alloc::sync::Arc;
use crate::println;
pub use info::context::TaskContext;
use info::task_struct::{TaskStatus, TaskStruct};
use loader::{get_app_data, get_num_app};
pub use loader::load_apps;
use manager::add_task;
use proc::{current_task, run_tasks, schedule, take_current_task, PROCESSOR};

use crate::trap::TrapContext;

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.get_user_token();
    token
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

pub fn exit_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner.borrow_mut();
    task_inner.status = TaskStatus::Exited;
    drop(task_inner);
    drop(task);

    let mut empty_ctx = TaskContext::default();
    schedule(&mut empty_ctx as *mut TaskContext)
}

pub fn add_all_tasks() {
    println!("[Warning] Kernel is trying to add all tasks into waiting queue!");
    let num_app = get_num_app();
    for i in 0..num_app {
        add_task(Arc::new(TaskStruct::new(get_app_data(i))))
    }
}

pub fn change_program_brk(size: i32) -> Option<usize> {
    if let Some(task) = PROCESSOR.borrow_mut().current() {
        let mut task_inner = task.inner.borrow_mut();
        task_inner.mm.change_brk(size)
    } else {
        None
    }
}
