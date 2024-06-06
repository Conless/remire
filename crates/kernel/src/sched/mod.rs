// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::sync::Arc;
use proc::{schedule, take_current_task};
use scheduler::{add_thread, remove_process};
use thread_info::ThreadInfo;

use crate::{log, sbi::shutdown, task::exit};

pub mod proc;
pub mod scheduler;
mod switch;
mod thread_info;

pub fn suspend_current_and_run_next() {
    let thread = take_current_task().unwrap();
    let mut thread_info = thread.borrow_mut();
    let thread_info_ptr = &mut *thread_info as *mut ThreadInfo;

    drop(thread_info);
    add_thread(thread);
    schedule(thread_info_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let thread = take_current_task().unwrap();
    let pid = thread.borrow_mut().pid;
    log!(
        "[kernel] Task {} exit with exit_code {} ...",
        pid,
        exit_code
    );

    if pid == 1 {
        log!(
            "[kernel] Init process exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            shutdown(true)
        } else {
            shutdown(false)
        }
    }
    
    remove_process(pid);
    assert!(Arc::strong_count(&thread) == 1);
    log!("[kernel] Calling task_struct->exit...");
    exit(pid, exit_code);

    let mut empty_ctx = ThreadInfo::default();
    schedule(&mut empty_ctx as *mut ThreadInfo)
}
