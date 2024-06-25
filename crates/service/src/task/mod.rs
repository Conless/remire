// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

extern crate alloc;

use alloc::sync::Arc;
use info::task_struct::TaskStruct;
use ksync::msg::task::PM2Kernel;
use manager::TASK_MANAGER;

mod info;
mod manager;
mod msg;
mod pid;

pub use msg::*;

pub fn fork(pid: usize, new_token: usize) -> usize {
    TASK_MANAGER.borrow_mut().fork(pid, new_token)
}

pub fn exec(pid: usize, new_token: usize) {
    TASK_MANAGER.borrow_mut().exec(pid, new_token)
}

pub fn waitpid(pid: usize, child_pid: isize) -> (isize, i32) {
    TASK_MANAGER.borrow_mut().waitpid(pid, child_pid)
}

pub fn exit(pid: usize, exit_code: i32, msg_helper: impl Fn(PM2Kernel)) {
    TASK_MANAGER.borrow_mut().exit(pid, exit_code, msg_helper)
}

pub fn init_task_manager(init_token: usize) -> usize {
    let init_task = Arc::new(TaskStruct::init(init_token));
    let init_task_pid = init_task.pid.0;
    if init_task_pid != 1 {
        panic!("init task pid is not 1");
    }
    TASK_MANAGER.borrow_mut().add(init_task);
    init_task_pid
}
