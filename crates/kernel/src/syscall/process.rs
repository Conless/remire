// Copyright (c) 2024 Conless Pan

use alloc::sync::Arc;

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use crate::task::loader::get_app_data_by_name;
use crate::task::manager::add_task;
use crate::trap::get_time_ms;
use crate::{
    mm::{translated_byte_buffer, translated_ptr, translated_str},
    log,
    task::{
        current_task, current_user_token, exit_current_and_run_next, suspend_current_and_run_next,
    },
    trap::get_time,
};

/// Exit the current application
///
/// This function will print the exit code of the application and run the next application.
pub fn sys_exit(exit_code: i32) -> ! {
    log!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next(exit_code);
    unreachable!()
}

pub fn sys_yield() -> isize {
    // log!("[kernel] Yield to next task");
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_task_pid = new_task.pid.0;
    let new_task_trap_ctx = new_task.get_trap_ctx();
    new_task_trap_ctx.regs[10] = 0; // fork return 0 in child process
    add_task(new_task);
    new_task_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();

    let mut inner = task.inner.borrow_mut();
    if !inner
        .children
        .iter()
        .any(|p| pid == -1 || pid as usize == p.pid.0)
    {
        return -1;
    }

    let pair = inner.children.iter().enumerate().find_map(|(idx, p)| {
        let (is_zombie, exit_code) = p.is_zombie();
        if is_zombie && (pid == -1 || pid as usize == p.pid.0) {
            Some((idx, exit_code))
        } else {
            None
        }
    });

    if let Some((idx, exit_code)) = pair {
        let child = inner.children.remove(idx);
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.pid.0;
        unsafe { *translated_ptr(inner.mm.token(), exit_code_ptr) = exit_code };
        found_pid as isize
    } else {
        -2
    }
}
