// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::sync::Arc;
use ksync::UPSafeCell;
use lazy_static::lazy_static;

use crate::{mm::get_trap_ctx, trap::TrapContext};

use super::{switch::__switch, thread_info::ThreadInfo};

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> =
        unsafe { UPSafeCell::new(Processor::default()) };
}

#[derive(Default)]
pub struct Processor {
    current: Option<Arc<UPSafeCell<ThreadInfo>>>,
    scheduler: ThreadInfo,
}

impl Processor {
    pub fn take_current(&mut self) -> Option<Arc<UPSafeCell<ThreadInfo>>> {
        self.current.take()
    }

    pub fn current(&self) -> Option<Arc<UPSafeCell<ThreadInfo>>> {
        self.current.clone()
    }

    pub fn set_current(&mut self, thread: Arc<UPSafeCell<ThreadInfo>>) {
        self.current = Some(thread);
    }

    pub fn scheduler(&mut self) -> *mut ThreadInfo {
        &mut self.scheduler as *mut ThreadInfo
    }
}

pub fn take_current_task() -> Option<Arc<UPSafeCell<ThreadInfo>>> {
    PROCESSOR.borrow_mut().take_current()
}

pub fn current_task() -> Option<Arc<UPSafeCell<ThreadInfo>>> {
    PROCESSOR.borrow_mut().current()
}

pub fn current_pid() -> usize {
    PROCESSOR.borrow_mut().current().unwrap().borrow_mut().pid
}

pub fn current_user_token() -> usize {
    PROCESSOR.borrow_mut().current().unwrap().borrow_mut().token
}

pub fn set_user_token(token: usize) {
    PROCESSOR.borrow_mut().current().unwrap().borrow_mut().token = token;
}

pub fn current_trap_ctx() -> &'static mut TrapContext {
    let token = current_user_token();
    get_trap_ctx(token)
}

pub fn schedule(switched_thread: *mut ThreadInfo) {
    let mut processor = PROCESSOR.borrow_mut();
    let next_thread = processor.scheduler();
    drop(processor);
    unsafe {
        __switch(switched_thread, next_thread);
    }
}
