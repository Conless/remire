// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::{
    collections::{BTreeMap, VecDeque},
    sync::Arc,
};
use lazy_static::lazy_static;

use ksync::UPSafeCell;

use crate::mm::remove_user_space;

use super::info::task_struct::{TaskStatus, TaskStruct};

/// Inner data structure of the task manager
#[derive(Default)]
pub struct TaskManager {
    tasks: BTreeMap<usize, Arc<TaskStruct>>,
}

lazy_static! {
    /// Global task manager
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe {
            UPSafeCell::new(TaskManager::default())
        };
}

impl TaskManager {
    pub fn add(&mut self, task: Arc<TaskStruct>) {
        self.tasks.insert(task.pid.0, task);
    }

    pub fn remove(&mut self, pid: usize) {
        self.tasks.remove(&pid);
    }

    pub fn fork(&mut self, pid: usize) -> (usize, usize) {
        let task = self.tasks.get(&pid).unwrap().clone();
        let new_task = task.fork();
        let (pid, token) = (new_task.pid.0, new_task.inner.borrow_mut().mm.0);
        self.add(new_task.clone());
        (pid, token)
    }

    pub fn exec(&mut self, pid: usize, app_name: &str) -> (isize, usize) {
        let task = self.tasks.get(&pid).unwrap().clone();
        let ret = task.exec(app_name);
        let token = task.inner.borrow_mut().mm.0;
        (ret, token)
    }

    pub fn waitpid(&mut self, pid: usize, child_pid: isize) -> (isize, i32) {
        let task = self.tasks.get(&pid).unwrap().clone();

        let mut inner = task.inner.borrow_mut();
        if !inner
            .children
            .iter()
            .any(|p| child_pid == -1 || child_pid as usize == p.pid.0)
        {
            return (-1, 0);
        }

        let pair = inner.children.iter().enumerate().find_map(|(idx, p)| {
            let (is_zombie, exit_code) = p.is_zombie();
            if is_zombie && (child_pid == -1 || child_pid as usize == p.pid.0) {
                Some((idx, exit_code))
            } else {
                None
            }
        });

        if let Some((idx, exit_code)) = pair {
            let child = inner.children.remove(idx);
            assert_eq!(Arc::strong_count(&child), 1);
            let found_pid = child.pid.0;
            (found_pid as isize, exit_code)
        } else {
            (-2, 0)
        }
    }

    pub fn exit(&mut self, pid: usize, exit_code: i32) {
        let task = self.tasks.get(&pid).unwrap().clone();
        let mut task_inner = task.inner.borrow_mut();
        task_inner.status = TaskStatus::Zombie(exit_code);

        {
            let init_task = self.tasks.get(&1).unwrap().clone();
            let mut init_task_inner = init_task.inner.borrow_mut();
            for child in task_inner.children.iter() {
                child.inner.borrow_mut().parent = Some(Arc::downgrade(&init_task));
                init_task_inner.children.push(child.clone());
            }
        }

        task_inner.children.clear();
        self.tasks.remove(&pid);
        remove_user_space(task_inner.mm.0);
        drop(task_inner);
        drop(task);
    }
}
