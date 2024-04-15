// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod context;
mod switch;
mod loader;
mod manager;

pub use context::TaskContext;
pub use loader::load_apps;

use self::manager::TASK_MANAGER;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

/// task control block structure
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub ctx: TaskContext,
}

pub fn run_first_task() -> ! {
    TASK_MANAGER.run_first_task()
}

fn suspend_current_task() {
    TASK_MANAGER.suspend();
}

fn exit_current_task() {
    TASK_MANAGER.exit();
}

pub fn suspend_to_next() {
    suspend_current_task();
    run_next_task();
}

pub fn exit_to_next() {
    exit_current_task();
    run_next_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

