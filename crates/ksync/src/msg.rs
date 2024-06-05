// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::mem::size_of;

use spin::RwLock;

pub trait GetID {
    fn get_id(&self) -> usize;
}

#[repr(align(4096))]
#[repr(C)]
pub struct MsgQueueInner<T, const N: usize>
where
    T: Sized + Default + Copy + GetID,
{
    head: usize,
    tail: usize,
    msgs: [T; N],
    size: usize,
    magic: usize,
}

pub type MsgQueue<T, const N: usize> = RwLock<MsgQueueInner<T, N>>;

impl<T, const N: usize> Default for MsgQueueInner<T, N>
where
    T: Sized + Default + Copy + GetID,
{
    fn default() -> Self {
        if size_of::<MsgQueue<T, N>>() > 4096 {
            panic!("MsgQueue is too large");
        }
        Self {
            head: 0,
            tail: 0,
            msgs: [T::default(); N],
            size: 0,
            magic: 0xac05e, // Represents "acore"
        }
    }
}

impl<T, const N: usize> MsgQueueInner<T, N>
where
    T: Sized + Default + Copy + GetID,
{
    /// # Safety
    /// The caller must ensure that the pointer is valid and properly aligned.
    pub unsafe fn from_ptr(ptr: *mut u8) -> &'static mut Self {
        let result = &mut *(ptr as *mut Self);
        if result.magic != 0xac05e {
            panic!("Invalid magic number");
        }
        if (result.tail - result.head + N) % N != result.size {
            panic!("Invalid size");
        }
        result
    }

    pub fn push(&mut self, msg: T) -> bool {
        if self.size == N {
            return false;
        }
        self.msgs[self.tail] = msg;
        self.tail = (self.tail + 1) % N;
        self.size += 1;
        true
    }

    pub fn test_id(&self, id: isize) -> bool {
        if self.size == 0 {
            return false;
        }
        let msg = self.msgs[self.head];
        id == -1 || id as usize == msg.get_id()
    }

    pub fn pop_id(&mut self, id: isize) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let msg = self.msgs[self.head];
        if id != -1 && id as usize != msg.get_id() {
            return None;
        }
        self.head = (self.head + 1) % N;
        self.size -= 1;
        Some(msg)
    }
}
