// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
#![no_std]
#![no_main]

extern crate service;

use ksync::task::{Kernel2PM, PM2Kernel};
use service::{msg::{recv_msg, send_msg}, task::{exec, exit, fork, init_task_manager}};

#[no_mangle]
pub fn main() -> i32 {
    loop {
        let msg = recv_msg(-1);
        match msg {
            Kernel2PM::Init { id, token } => {
                init_task_manager(token);
            },
            Kernel2PM::Fork { id, pid, token } => {
                let new_pid = fork(pid, token);
                send_msg(PM2Kernel::ForkReply { id, pid, child_pid: new_pid });
            },
            Kernel2PM::Exec { id: _, pid, token } => {
                exec(pid, token);
            },
            Kernel2PM::Exit { id: _, pid, exit_code } => {
                exit(pid, exit_code);
            },
            _ => panic!("Invalid message"),
        }
    }
}