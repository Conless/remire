// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use core::sync::atomic::{AtomicIsize, Ordering};

use super::queue::{MsgQueue, MsgWrapper};

pub struct MsgIOQueue<I, O, const N: usize, const M: bool>
where
    I: Copy + Default,
    O: Copy + Default,
{
    send_id: AtomicIsize,
    send_port: *mut MsgQueue<O, N>,
    recv_port: *mut MsgQueue<I, N>,
    yield_: Option<fn()>,
}

impl<I, O, const N: usize, const M: bool> MsgIOQueue<I, O, N, M>
where I: Copy + Default, O: Copy + Default {
    pub const fn default() -> Self {
        let send_id = if M { 1 } else { -1 };
        Self {
            send_id: AtomicIsize::new(send_id),
            send_port: core::ptr::null_mut() as *mut MsgQueue<O, N>,
            recv_port: core::ptr::null_mut() as *mut MsgQueue<I, N>,
            yield_: None,
        }
    }
    
    pub const unsafe fn new(send_port: usize, recv_port: usize, yield_: fn()) -> Self {
        Self {
            send_id: AtomicIsize::new(if M { 1 } else { -1 }),
            send_port: send_port as *mut MsgQueue<O, N>,
            recv_port: recv_port as *mut MsgQueue<I, N>,
            yield_: Some(yield_),
        }
    }

    pub unsafe fn init(&mut self, send_port: usize, recv_port: usize, yield_: fn()) {
        self.send_port = send_port as *mut MsgQueue<O, N>; 
        self.recv_port = recv_port as *mut MsgQueue<I, N>;
        self.yield_ = Some(yield_);
    }

    pub unsafe fn send(&self, msg: O) -> isize {
        let msg_id = if M {
            self.send_id.fetch_add(1, Ordering::Relaxed)
        } else {
            self.send_id.fetch_sub(1, Ordering::Relaxed)
        };
        let msg = MsgWrapper { msg, id: msg_id };
        loop {
            let send_port_ptr = &mut *self.send_port;
            let mut send_port = send_port_ptr.write();
            if send_port.push(msg) {
                return msg_id;
            } else {
                drop(send_port);
                self.yield_.unwrap()();
            }
        }
    }
    
    pub unsafe fn reply(&self, id: isize, msg: O) {
        let msg = MsgWrapper { msg, id };
        loop {
            let send_port_ptr = &mut *self.send_port;
            let mut send_port = send_port_ptr.write();
            if send_port.push(msg) {
                break;
            } else {
                drop(send_port);
                self.yield_.unwrap()();
            }
        }
    }

    unsafe fn try_recv(&self, id: isize) -> bool {
        let recv_port_ptr = &mut *self.recv_port;
        let recv_port = recv_port_ptr.read();
        recv_port.test_id(id)
    }

    pub unsafe fn recv(&self, id: isize) -> I {
        loop {
            if self.try_recv(id) {
                break;
            } else {
                self.yield_.unwrap()();
            }
        }
        let recv_port_ptr = &mut *self.recv_port;
        let mut recv_port = recv_port_ptr.write();
        recv_port.pop_id(id).unwrap().msg
    }
}
