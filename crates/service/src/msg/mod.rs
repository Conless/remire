// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use handler::MsgHandler;
use id::init_msg_id_allocator;
use ksync::msg::GetID;
use ksync::task::{Kernel2PM, PM2Kernel};

mod handler;
pub mod id;

static mut MSG_HANDLER: MsgHandler = MsgHandler::new();

pub fn init_msg_handler(send_va: usize, recv_va: usize, start_id: usize, end_id: usize) {
    unsafe {
        MSG_HANDLER.init(send_va, recv_va);
    }
    init_msg_id_allocator(start_id, end_id);
}

pub fn send_msg(msg: PM2Kernel) {
    unsafe {
        MSG_HANDLER.send(msg);
    }
}

pub fn send_msg_and_wait(msg: PM2Kernel) -> Kernel2PM {
    send_msg(msg);
    recv_msg(msg.get_id() as isize)
}

pub fn recv_msg(id: isize) -> Kernel2PM {
    unsafe {
        MSG_HANDLER.recv(id)
    }
}

