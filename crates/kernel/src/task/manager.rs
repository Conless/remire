// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::{collections::VecDeque, sync::Arc};
use lazy_static::lazy_static;

use crate::sync::UPSafeCell;

use super::info::task_struct::TaskStruct;

/// Inner data structure of the task manager
#[derive(Default)]
pub struct TaskManager {
    tasks: VecDeque<Arc<TaskStruct>>, // List of task control blocks
}

lazy_static! {
    /// Global task manager
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = 
        unsafe { UPSafeCell::new(TaskManager::default()) };
}

impl TaskManager {
    pub fn add(&mut self, task: Arc<TaskStruct>) {
        self.tasks.push_back(task);
    }

    pub fn pop(&mut self) -> Option<Arc<TaskStruct>> {
        self.tasks.pop_front()
    }
}

pub fn add_task(task: Arc<TaskStruct>) {
    TASK_MANAGER.borrow_mut().add(task);
}

pub fn pop_task() -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.borrow_mut().pop()
}
