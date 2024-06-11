// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub mod pm;

pub fn init_services() {
    pm::init_pm();
}

pub fn reply_services() {
    pm::reply();
}

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
            MSG_QUEUE.spin_recv(id)
        }
    };
}

#[macro_export]
macro_rules! resolve_msg {
    () => {
        unsafe { MSG_QUEUE.resolve() }
    };
}

#[macro_export]
macro_rules! recv_msg {
    ($id:expr) => {
        unsafe { MSG_QUEUE.spin_recv($id) }
    };
}
