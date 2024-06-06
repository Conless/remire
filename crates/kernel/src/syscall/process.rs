// Copyright (c) 2024 Conless Pan

use alloc::sync::Arc;
use ksync::UPSafeCell;

use crate::mm::get_trap_ctx;
use crate::sched::proc::{current_pid, current_user_token, set_user_token};
use crate::sched::scheduler::add_process;
use crate::sched::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::task::{exec, fork, waitpid};
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use crate::trap::get_time_ms;
use crate::{
    mm::{translated_ptr, translated_str},
    log,
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
    current_pid() as isize
}

pub fn sys_fork() -> isize {
    let (new_task_pid, new_task_token) = fork(current_pid());
    let new_task_trap_ctx = get_trap_ctx(new_task_token);
    new_task_trap_ctx.regs[10] = 0; // fork return 0 in child process
    
    add_process(new_task_pid, new_task_token);
    new_task_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let current_pid = current_pid();
    let current_token = current_user_token();
    let path = translated_str(current_token, path);
    let (result, token) = exec(current_pid, path.as_str());
    set_user_token(token);
    result
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let (result, exit_code) = waitpid(current_pid(), pid);
    if result >= 0 {
        unsafe { *translated_ptr(current_user_token(), exit_code_ptr) = exit_code };
    }
    result
}
