// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod context;
mod switch;
mod loader;
mod manager;

pub use context::TaskContext;

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
