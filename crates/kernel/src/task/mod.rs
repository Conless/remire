// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::sync::Arc;
use info::task_struct::TaskStruct;
use manager::TASK_MANAGER;

mod info;
mod manager;
mod pid;

pub fn fork(pid: usize, token: usize) -> usize {
    TASK_MANAGER.borrow_mut().fork(pid, token)
}

pub fn exec(pid: usize, token: usize) {
    TASK_MANAGER.borrow_mut().exec(pid, token)
}

pub fn waitpid(pid: usize, child_pid: isize) -> (isize, i32) {
    TASK_MANAGER.borrow_mut().waitpid(pid, child_pid)
}

pub fn exit(pid: usize, exit_code: i32) {
    TASK_MANAGER.borrow_mut().exit(pid, exit_code)
}

pub fn init_task_manager() -> (usize, usize) {
    let init_task = Arc::new(TaskStruct::new("initproc"));
    let init_task_pid = init_task.pid.0;
    if init_task_pid != 1 {
        panic!("init task pid is not 1");
    }
    let init_task_token = init_task.inner.borrow_mut().mm.0;
    TASK_MANAGER.borrow_mut().add(init_task);
    (init_task_pid, init_task_token)
}
