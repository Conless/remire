// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use ksync::msg::{
    task::{Kernel2PM, PM2Kernel},
    PM2KernelQueue,
};

use crate::{config::{SERVICE_RECV_PORT, SERVICE_SEND_PORT}, yield_};


fn yield_current_and_run_next() {
    yield_();
}

static mut MSG_QUEUE: PM2KernelQueue =
    unsafe { PM2KernelQueue::new(SERVICE_SEND_PORT, SERVICE_RECV_PORT, yield_current_and_run_next) };

pub fn send_msg(msg: PM2Kernel) {
    unsafe {
        MSG_QUEUE.send(msg);
    }
}

pub fn send_msg_and_wait(msg: PM2Kernel) -> Kernel2PM {
    unsafe {
        let id = MSG_QUEUE.send(msg);
        MSG_QUEUE.recv(id)
    }
}

pub fn recv_msg(id: isize) -> Kernel2PM {
    unsafe { MSG_QUEUE.recv(id) }
}
