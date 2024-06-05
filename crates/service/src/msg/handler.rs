// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use core::sync::atomic::{AtomicPtr, Ordering};

use ksync::task::{Kernel2PM, Kernel2PMQueue, PM2Kernel, PM2KernelQueue};

use crate::yield_;

pub struct MsgHandler {
    send_port: AtomicPtr<PM2KernelQueue>,
    recv_port: AtomicPtr<Kernel2PMQueue>,
}

impl MsgHandler {
    pub const fn new() -> Self {
        Self {
            send_port: AtomicPtr::new(core::ptr::null_mut() as *mut PM2KernelQueue),
            recv_port: AtomicPtr::new(core::ptr::null_mut() as *mut Kernel2PMQueue),
        }
    }

    pub unsafe fn init(&mut self, send_port: usize, recv_port: usize) {
        self.send_port = AtomicPtr::new(send_port as *mut PM2KernelQueue);
        self.recv_port = AtomicPtr::new(recv_port as *mut Kernel2PMQueue);
    }

    pub unsafe fn send(&mut self, msg: PM2Kernel) {
        loop {
            let send_port_ptr = &mut *self.send_port.load(Ordering::Relaxed);
            let mut send_port = send_port_ptr.write();
            if send_port.push(msg) {
                break;
            } else {
                drop(send_port);
                yield_();
            }
        }
    }

    unsafe fn try_recv(&mut self, id: isize) -> bool {
        let recv_port_ptr = &mut *self.recv_port.load(Ordering::Relaxed);
        let recv_port = recv_port_ptr.read();
        recv_port.test_id(id)
    }

    pub unsafe fn recv(&mut self, id: isize) -> Kernel2PM {
        loop {
            if self.try_recv(id) {
                break;
            } else {
                yield_();
            }
        }
        let recv_port_ptr = &mut *self.recv_port.load(Ordering::Relaxed);
        let mut recv_port = recv_port_ptr.write();
        recv_port.pop_id(id).unwrap()
    }
}
