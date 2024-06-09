// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::sched::suspend_current_and_run_next;

mod pm;

#[macro_export]
macro_rules! send_msg {
    ($msg:expr) => {
        unsafe { MSG_QUEUE.send($msg) }
    };
}

#[macro_export]
macro_rules! send_msg_and_wait {
    ($msg:expr) => {
        unsafe {
            let id = MSG_QUEUE.send($msg);
            MSG_QUEUE.recv(id)
        }
    };
}

#[macro_export]
macro_rules! recv_msg {
    ($id:expr) => {
        unsafe { MSG_QUEUE.recv($id) }
    };
}
