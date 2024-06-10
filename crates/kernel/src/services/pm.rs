// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use ksync::msg::{
    task::{Kernel2PM, PM2Kernel},
    Kernel2PMQueue,
};

use crate::{
    loader::get_service_data_by_name, mm::new_service, sched::scheduler::add_service, send_msg,
    send_msg_and_wait, syscall::sys_yield,
};

static mut MSG_QUEUE: Kernel2PMQueue = Kernel2PMQueue::default();

pub fn yield_current_and_run_next() {
    sys_yield();
}

pub fn init_pm() {
    let pm_data = get_service_data_by_name("pm").unwrap();
    let (token, recv_pa, send_pa) = new_service(pm_data);
    unsafe {
        MSG_QUEUE.init(send_pa, recv_pa, yield_current_and_run_next);
    }
    add_service(token);
}

pub fn fork(pid: usize, token: usize) -> usize {
    if let PM2Kernel::ForkReply { child_pid } = send_msg_and_wait!(Kernel2PM::Fork { pid, token }) {
        child_pid
    } else {
        panic!("Fork failed");
    }
}

pub fn exec(pid: usize, new_token: usize) {
    send_msg!(Kernel2PM::Exec {
        pid,
        token: new_token
    });
}

pub fn waitpid(pid: usize, child_pid: isize) -> (isize, i32) {
    if let PM2Kernel::WaitPIDReply { result, exit_code } =
        send_msg_and_wait!(Kernel2PM::WaitPID { pid, child_pid })
    {
        (result, exit_code)
    } else {
        panic!("Waitpid failed");
    }
}

pub fn exit(pid: usize, exit_code: i32) {
    send_msg!(Kernel2PM::Exit { pid, exit_code });
}
