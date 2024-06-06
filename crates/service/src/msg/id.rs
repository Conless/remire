// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use allocator::StackAllocator;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

lazy_static! {
    pub static ref MSG_ID_ALLOCATOR: Mutex<StackAllocator> = Mutex::new(StackAllocator::default());
}

pub fn init_msg_id_allocator(start_id: usize, end_id: usize) {
    MSG_ID_ALLOCATOR.lock().init(start_id, end_id);
}

pub fn alloc_msg_id() -> Option<usize> {
    MSG_ID_ALLOCATOR.lock().alloc()
}

pub fn dealloc_msg_id(id: usize) {
    MSG_ID_ALLOCATOR.lock().dealloc(id)
}
