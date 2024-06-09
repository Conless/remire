// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use alloc::{collections::{BTreeMap, VecDeque}, string::{String, ToString}, sync::Arc};
use ksync::UPSafeCell;
use lazy_static::lazy_static;
use super::{proc::PROCESSOR, switch::__switch};
use super::thread_info::ThreadInfo;

use crate::log;

lazy_static! {
    pub static ref SCHEDULER: UPSafeCell<Scheduler> =
        unsafe { UPSafeCell::new(Scheduler::default()) };
}

#[derive(Default)]
pub struct Scheduler {
    threads: VecDeque<Arc<UPSafeCell<ThreadInfo>>>,
    services: BTreeMap<String, Arc<UPSafeCell<ThreadInfo>>>,
}

impl Scheduler {
    fn add_process(&mut self, thread: Arc<UPSafeCell<ThreadInfo>>) {
        self.threads.push_back(thread.clone());
    }
    
    fn add_service(&mut self, service_name: &str, thread: Arc<UPSafeCell<ThreadInfo>>) {
        self.services.insert(service_name.to_string(), thread.clone());
    }
    
    pub fn add_thread(&mut self, thread: Arc<UPSafeCell<ThreadInfo>>) {
        assert!(thread.borrow_mut().pid != 0, "Add a thread with pid 0");
        self.threads.push_back(thread);
    }

    pub fn pop(&mut self) -> Option<Arc<UPSafeCell<ThreadInfo>>> {
        self.threads.pop_front()
    }
}

pub fn add_process(pid: usize, token: usize) {
    let thread = Arc::new(unsafe { UPSafeCell::new(ThreadInfo::new(pid, token)) });
    SCHEDULER.borrow_mut().add_process(thread);
}

pub fn add_service(service_name: &str, token: usize) {
    let thread = Arc::new(unsafe { UPSafeCell::new(ThreadInfo::new(0, token)) });
    SCHEDULER.borrow_mut().add_service(service_name, thread);
}

pub fn add_thread(thread: Arc<UPSafeCell<ThreadInfo>>) {
    SCHEDULER.borrow_mut().add_thread(thread);
}

pub fn pop_thread() -> Option<Arc<UPSafeCell<ThreadInfo>>> {
    SCHEDULER.borrow_mut().pop()
}

pub fn start_schedule() -> ! {
    loop {
        let mut processor = PROCESSOR.borrow_mut();
        if let Some(thread) = pop_thread() {
            let scheduler = processor.scheduler();
            let mut next_thread = thread.borrow_mut();
            let next_thread_pid = next_thread.pid;
            let next_thread_ptr = &mut *next_thread as *mut ThreadInfo;
            let mut current_sp: usize;
            unsafe {
                asm!("mv {}, sp", out(reg) current_sp);
            }
            log!(
                "[kernel] Switch to task {}:{:x} with sp {:x} ...",
                next_thread_pid,
                next_thread.get_sp(),
                current_sp,
            );
            // unsafe {
                // asm!("lw {}, 0x0({})", out(reg) current_sp, in(reg) next_thread.sp - 4);
            // }
            log!(
                "[kernel] Switch to task {} with data {:x} ...",
                next_thread_pid,
                current_sp,
            );
            drop(next_thread);

            processor.set_current(thread);
            drop(processor);
            // log!("Here");
            unsafe {
                __switch(scheduler, next_thread_ptr);
            }
        }
    }
}