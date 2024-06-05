// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::{sync::Arc, sync::Weak, vec::Vec};
use ksync::UPSafeCell;

use crate::{
    // mm::{get_kernel_stack, new_user_space_from_token},
    task::pid::{alloc_pid, PIDGuard},
};

use super::mm_guard::MMGuard;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie(i32), // exit code
}

/// task control block structure
pub struct TaskStruct {
    pub pid: PIDGuard,
    pub inner: UPSafeCell<TaskStructInner>,
}

pub struct TaskStructInner {
    pub status: TaskStatus,
    pub mm: MMGuard,
    pub parent: Option<Weak<TaskStruct>>,
    pub children: Vec<Arc<TaskStruct>>,
}

impl TaskStruct {
    pub fn is_zombie(&self) -> (bool, i32) {
        match self.inner.borrow_mut().status {
            TaskStatus::Zombie(code) => (true, code),
            _ => (false, 0),
        }
    }

    pub fn init(token: usize) -> Self {
        let mm = MMGuard::from_token(token);
        let pid_guard = alloc_pid().unwrap();

        let task_struct = Self {
            pid: pid_guard,
            inner: unsafe {
                UPSafeCell::new(TaskStructInner {
                    status: TaskStatus::Ready,
                    mm,
                    parent: None,
                    children: Vec::new(),
                })
            },
        };
        task_struct
    }

    pub fn fork(self: &Arc<TaskStruct>, new_token: usize) -> Arc<TaskStruct> {
        let mut parent_inner = self.inner.borrow_mut();
        let mm = MMGuard::from_token(new_token);
        let pid_guard = alloc_pid().unwrap();
        log!(
            "[kernel] Fork new task {} from task {}",
            pid_guard.0,
            self.pid.0
        );
        let task_struct = Arc::new(TaskStruct {
            pid: pid_guard,
            inner: unsafe {
                UPSafeCell::new(TaskStructInner {
                    status: TaskStatus::Ready,
                    mm,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                })
            },
        });

        parent_inner.children.push(task_struct.clone());
        task_struct
    }

    pub fn exec(&self, new_token: usize) {
        self.inner.borrow_mut().mm = MMGuard::from_token(new_token);
    }
}
