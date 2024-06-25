// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use ksync::msg::{fs::{FS2Kernel, Kernel2FS}, FS2KernelPort};

use crate::{config::{SERVICE_RECV_PORT, SERVICE_SEND_PORT}, yield_};


fn yield_current_and_run_next() {
    yield_();
}

static mut MSG_QUEUE: FS2KernelPort =
    unsafe { FS2KernelPort::new(SERVICE_SEND_PORT, SERVICE_RECV_PORT, yield_current_and_run_next) };

pub fn reply_msg(id: isize, msg: FS2Kernel) {
    unsafe {
        MSG_QUEUE.reply(id, msg);
    }
}

pub fn send_msg(msg: FS2Kernel) {
    unsafe {
        MSG_QUEUE.send(msg);
    }
}

pub fn send_msg_and_wait(msg: FS2Kernel) -> Kernel2FS {
    unsafe {
        let id = MSG_QUEUE.send(msg);
        MSG_QUEUE.spin_recv(id).1
    }
}

pub fn recv_msg(id: isize) -> (isize, Kernel2FS) {
    unsafe { MSG_QUEUE.spin_recv(id) }
}
