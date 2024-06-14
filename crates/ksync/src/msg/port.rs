// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use core::sync::atomic::{AtomicIsize, Ordering};

use super::queue::{MsgQueue, MsgWrapper};

pub struct MsgPort<I, O, const N: usize, const M: bool>
where
    I: Copy + Default,
    O: Copy + Default,
{
    send_id: AtomicIsize,
    send_port: *mut MsgQueue<O, N>,
    recv_port: *mut MsgQueue<I, N>,
    yield_: Option<fn()>,
}

impl<I, O, const N: usize, const M: bool> MsgPort<I, O, N, M>
where
    I: Copy + Default,
    O: Copy + Default,
{
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

    unsafe fn try_recv(&self, test_func: &dyn Fn(isize) -> bool) -> Option<isize> {
        let recv_port_ptr = &mut *self.recv_port;
        let recv_port = recv_port_ptr.read();
        let peak_id = recv_port.peak_id();
        if peak_id != 0 && test_func(peak_id) {
            Some(peak_id)
        } else {
            None
        }
    }

    pub unsafe fn spin_recv(&self, id: isize) -> (isize, I) {
        loop {
            if let Some(id) = self.try_recv(&|a| id == 0 || id == a) {
                let recv_port_ptr = &mut *self.recv_port;
                let mut recv_port = recv_port_ptr.write();
                let msg = recv_port.pop_id(id).unwrap();
                return (msg.id, msg.msg);
            } else {
                self.yield_.unwrap()();
            }
        }
    }

    pub unsafe fn resolve(&self) -> Option<(isize, I)> {
        let test_func = if M { |a| a < 0 } else { |a| a > 0 };
        if let Some(id) = self.try_recv(&test_func) {
            let recv_port_ptr = &mut *self.recv_port;
            let mut recv_port = recv_port_ptr.write();
            let msg = recv_port.pop_id(id).unwrap();
            Some((msg.id, msg.msg))
        } else {
            None
        }
    }
}
