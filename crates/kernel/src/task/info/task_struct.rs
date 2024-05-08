// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::{sync::Weak, sync::Arc, vec::Vec};

use crate::{config::TRAP_CONTEXT, mm::{types::{PhysPageNum, VirtAddr}, MMStruct, KERNEL_SPACE}, log, stack::KernelStack, sync::UPSafeCell, task::pid::{alloc_pid, PIDGuard}, trap::{trap_handler, TrapContext}};

use super::context::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie(i32), // exit code
}

/// task control block structure
pub struct TaskStruct {
    pub pid: PIDGuard,
    pub stack: KernelStack,
    pub inner: UPSafeCell<TaskStructInner>,
}

pub struct TaskStructInner {
    pub status: TaskStatus,
    pub ctx: TaskContext,
    pub mm: MMStruct,
    pub ctx_ppn: PhysPageNum,
    pub parent: Option<Weak<TaskStruct>>,
    pub children: Vec<Arc<TaskStruct>>,
}

impl TaskStruct {
    pub fn get_trap_ctx(&self) -> &'static mut TrapContext {
        self.inner.borrow_mut().ctx_ppn.get_mut()
    }
    
    pub fn is_zombie(&self) -> (bool, i32) {
        match self.inner.borrow_mut().status {
            TaskStatus::Zombie(code) => (true, code),
            _ => (false, 0),
        }
    }

    pub fn get_user_token(&self) -> usize {
        self.inner.borrow_mut().mm.token()
    }

    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (mm, user_sp, entry_point) = MMStruct::new_app(elf_data);
        let pid_guard = alloc_pid().unwrap();
        let kernel_stack = KernelStack::new(&pid_guard);
        let kernel_stack_top = kernel_stack.get_top();
        let ctx_ppn = mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();

        let task_struct = Self {
            pid: pid_guard,
            stack: kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskStructInner {
                    status: TaskStatus::Ready,
                    ctx: TaskContext::restore(kernel_stack_top),
                    mm,
                    ctx_ppn,
                    parent: None,
                    children: Vec::new(),
                })
            }
        };

        // prepare TrapContext in user space
        let trap_ctx = task_struct.get_trap_ctx();
        *trap_ctx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.borrow_mut().token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_struct
    }

    pub fn change_program_brk(&mut self, size: i32) -> Option<usize> {
        self.inner.borrow_mut().mm.change_brk(size)
    }

    pub fn fork(self: &Arc<TaskStruct>) -> Arc<TaskStruct> {
        let mut parent_inner = self.inner.borrow_mut();
        let mm = parent_inner.mm.clone();
        let ctx_ppn = mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();
        let pid_guard = alloc_pid().unwrap();
        log!("[kernel] Fork new task {} from task {}", pid_guard.0, self.pid.0);
        let kernel_stack = KernelStack::new(&pid_guard);
        let kernel_stack_top = kernel_stack.get_top();
        let task_struct = Arc::new(
            TaskStruct {
                pid: pid_guard,
                stack: kernel_stack,
                inner: unsafe {
                    UPSafeCell::new(TaskStructInner {
                        status: TaskStatus::Ready,
                        ctx: TaskContext::restore(kernel_stack_top),
                        mm,
                        ctx_ppn,
                        parent: Some(Arc::downgrade(self)),
                        children: Vec::new(),
                    })
                }
            }
        );
        
        parent_inner.children.push(task_struct.clone());
        let trap_ctx = task_struct.get_trap_ctx();
        trap_ctx.sp = kernel_stack_top;
        task_struct
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let (mm, user_sp, entry_point) = MMStruct::new_app(elf_data);
        let mut inner = self.inner.borrow_mut();
        inner.mm = mm;
        inner.ctx_ppn = inner.mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();
        let trap_ctx = inner.ctx_ppn.get_mut();
        *trap_ctx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.borrow_mut().token(),
            self.stack.get_top(),
            trap_handler as usize,
        );
    }
}
