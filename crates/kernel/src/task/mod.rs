// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod context;
mod switch;
mod loader;
mod manager;

pub use context::TaskContext;
pub use loader::load_apps;

use crate::{addr::{MapPermission, MemorySet, PhysAddr, PhysPageNum, VirtAddr, KERNEL_SPACE}, config::{KERNEL_HEAP_SIZE, KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT}, stack::KERNEL_STACK, trap::{trap_handler, TrapContext}};

use self::manager::TASK_MANAGER;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

/// task control block structure
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub ctx: TaskContext,
    pub memory_set: MemorySet,
    pub ctx_ppn: PhysPageNum,
    pub size: usize,
}

impl TaskControlBlock {
    pub fn get_trap_ctx(&self) -> &'static mut TrapContext {
        self.ctx_ppn.get_mut()
    }
    
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }

    pub fn new(app_data: &[u8], app_id: usize) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::new_app(app_data);
        let ctx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap();
        let status = TaskStatus::Ready;

        let kernel_stack_top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
        let kernel_stack_bottom = kernel_stack_top - KERNEL_STACK_SIZE;
        
        KERNEL_SPACE
            .borrow_mut()
            .insert(
                kernel_stack_bottom.into(),
                kernel_stack_top.into(),
                MapPermission::R | MapPermission::W,
            );
        let task_control_block = Self {
            status,
            ctx: TaskContext::restore(kernel_stack_top),
            memory_set,
            ctx_ppn,
            size: user_sp,
        };
        // prepare TrapContext in user space
        let trap_ctx = task_control_block.get_trap_ctx();
        *trap_ctx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.borrow_mut().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }

}

pub fn run_first_task() -> ! {
    TASK_MANAGER.run_first_task()
}

fn suspend_current_task() {
    TASK_MANAGER.suspend();
}

fn exit_current_task() {
    TASK_MANAGER.exit();
}

pub fn suspend_to_next() {
    suspend_current_task();
    run_next_task();
}

pub fn exit_to_next() {
    exit_current_task();
    run_next_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_ctx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_ctx()
}

