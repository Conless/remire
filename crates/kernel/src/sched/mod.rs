// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::{collections::VecDeque, sync::Arc};
use ksync::UPSafeCell;
use lazy_static::lazy_static;
use proc::{current_pid, schedule, take_current_task, Processor, PROCESSOR};
use spin::Mutex;
use switch::__switch;
use thread_info::ThreadInfo;

use crate::{log, sbi::shutdown, task::exit};

pub mod proc;
mod switch;
mod thread_info;

lazy_static! {
    pub static ref SCHEDULER: UPSafeCell<Scheduler> =
        unsafe { UPSafeCell::new(Scheduler::default()) };
}

#[derive(Default)]
pub struct Scheduler {
    threads: VecDeque<Arc<UPSafeCell<ThreadInfo>>>,
}

impl Scheduler {
    pub fn add(&mut self, thread: Arc<UPSafeCell<ThreadInfo>>) {
        self.threads.push_back(thread);
    }

    pub fn pop(&mut self) -> Option<Arc<UPSafeCell<ThreadInfo>>> {
        self.threads.pop_front()
    }
}

pub fn add_process(pid: usize, token: usize) {
    let thread = Arc::new(unsafe { UPSafeCell::new(ThreadInfo::new(pid, token)) });
    SCHEDULER.borrow_mut().add(thread);
}

fn add_thread(thread: Arc<UPSafeCell<ThreadInfo>>) {
    SCHEDULER.borrow_mut().add(thread);
}

fn pop_thread() -> Option<Arc<UPSafeCell<ThreadInfo>>> {
    SCHEDULER.borrow_mut().pop()
}

pub fn start_schedule() -> ! {
    loop {
        let mut processor = PROCESSOR.borrow_mut();
        if let Some(thread) = pop_thread() {
            let scheduler = processor.scheduler();
            let mut next_thread = thread.borrow_mut();
            let next_thread_ptr = &mut *next_thread as *mut ThreadInfo;
            drop(next_thread);

            processor.set_current(thread);
            drop(processor);
            unsafe {
                __switch(scheduler, next_thread_ptr);
            }
        }
    }
}

pub fn suspend_current_and_run_next() {
    let thread = take_current_task().unwrap();
    let mut thread_info = thread.borrow_mut();
    let thread_info_ptr = &mut *thread_info as *mut ThreadInfo;

    drop(thread_info);
    add_thread(thread);
    schedule(thread_info_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let pid = current_pid();
    log!(
        "[kernel] Task {} exit with exit_code {} ...",
        pid,
        exit_code
    );

    if pid == 1 {
        log!(
            "[kernel] Init process exit with exit_code {} ...",
            exit_code
        );
        if exit_code != 0 {
            shutdown(true)
        } else {
            shutdown(false)
        }
    }
    
    exit(pid, exit_code);

    let mut empty_ctx = ThreadInfo::default();
    schedule(&mut empty_ctx as *mut ThreadInfo)
}
