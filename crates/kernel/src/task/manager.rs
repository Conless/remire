// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{println, sbi::shutdown, sync::UPSafeCell, task::{context::init_app, loader::get_num_app, TaskContext, TaskStatus}};

use super::{switch::__switch, TaskControlBlock};

/// Task manager of the kernel
/// 
/// Use `UPSafeCell` to wrap the inner data structure to provide interior mutability
pub struct TaskManager {
    num_app: usize,                      // Number of applications
    inner: UPSafeCell<TaskManagerInner>, // Wrapper for inner mutability
}

/// Inner data structure of the task manager
struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>, // List of task control blocks
    current_task: usize, // Unlike batch manager, current_task can appeared in any position
}

lazy_static! {
    /// Global task manager
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = Vec::new();
        for i in 0..num_app {
            // Init all the applications
            tasks.push(TaskControlBlock {
                status: TaskStatus::Ready,
                ctx: TaskContext::restore(init_app(i)),
            });
        }
        TaskManager {
            num_app,
            inner: unsafe { UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            })},
        }
    };
}

impl TaskManager {
    /// Suspend current task
    pub fn suspend(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Ready;
    }

    /// Exit current task
    pub fn exit(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Exited;
    }
    
    /// Change the running task to the next task found by `find_next_task`
    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            inner.tasks[next].status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].ctx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].ctx as *const TaskContext;
            
            // Drop local variables that must be dropped manually
            drop(inner);

            unsafe {
                __switch( // Switch context to the next task
                    current_task_cx_ptr,
                    next_task_cx_ptr,
                );
            }
        } else {
            println!("All applications completed!");
            shutdown(false)
        }
    }
    
    /// Find the next task to run
    /// 
    /// This function may be replaced by some other scheduling algorithms later?
    pub fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow_mut();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| {
                inner.tasks[*id].status == TaskStatus::Ready
            })
    }
    
    /// Run the first task
    pub fn run_first_task(&self) -> ! {
        let mut inner = self.inner.borrow_mut();
        let task = &mut inner.tasks[0];
        task.status = TaskStatus::Running;
        let task_ctx = &task.ctx as *const TaskContext;
        let mut empty_ctx = TaskContext::init();

        // Drop local variables that must be dropped manually
        drop(inner);

        unsafe {
            __switch(
                &mut empty_ctx as *mut TaskContext,
                task_ctx,
            );
        }
        unreachable!()
    }
}
