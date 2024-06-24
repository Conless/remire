// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod frame;
mod mm_struct;
mod page;
mod page_table;
mod translation;
mod vm_area;

use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use lazy_static::lazy_static;

use mm_struct::MMStruct;
use page::StepByOne;
use page_table::PageTable;

use ksync::UPSafeCell;

pub use frame::init_frame_allocator;
pub use page::VirtAddr;
pub use translation::*;
pub use vm_area::MapPermission;
use vm_area::{MapType, VMArea};

use crate::{
    config::{PAGE_SIZE, SERVICE_RECV_PORT, SERVICE_SEND_PORT, TRAP_CONTEXT}, log, trap::{trap_handler, TrapContext}
};

pub mod types {
    pub use super::frame::{PhysAddr, PhysPageNum};
}

lazy_static! {
  /// a memory set instance through lazy_static! managing kernel space
  pub static ref KERNEL_SPACE: Arc<UPSafeCell<MMStruct>> =
      Arc::new(unsafe { UPSafeCell::new(MMStruct::new_kernel()) });
  pub static ref USER_SPACES: UPSafeCell<BTreeMap<usize, MMStruct>> =
      unsafe { UPSafeCell::new(BTreeMap::new()) };
}

pub fn activate_kernel_space() {
    KERNEL_SPACE.borrow_mut().activate();
}

pub fn get_kernel_stack(token: usize) -> usize {
    let user_spaces = USER_SPACES.borrow_mut();
    let mm = user_spaces.get(&token).unwrap();
    let sp = mm.kernel_stack_top();
    if sp == 0 {
        panic!("kernel stack is not initialized");
    }
    sp
}

pub fn get_trap_ctx(token: usize) -> &'static mut TrapContext {
    let user_spaces = USER_SPACES.borrow_mut();
    let mm = user_spaces.get(&token).unwrap();
    let trap_ctx_ppn = mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();
    trap_ctx_ppn.get_mut()
}

pub fn new_user_space(elf_data: &[u8]) -> usize {
    let (mm, user_sp, entry_point) = MMStruct::new_app(elf_data);
    let trap_ctx_ppn = mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();
    let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
    *trap_ctx = TrapContext::app_init_context(
        entry_point,
        user_sp,
        KERNEL_SPACE.borrow_mut().token(),
        mm.kernel_stack_top(),
        trap_handler as usize,
    );
    let token = mm.token();
    log!("[kernel] New user space created: token = {:x}", token);
    USER_SPACES.borrow_mut().insert(token, mm);
    token
}

pub fn new_service(elf_data: &[u8]) -> (usize, usize, usize) {
    let (mut mm, user_sp, entry_point) = MMStruct::new_app(elf_data);
    let service_send_port = mm.alloc_port(SERVICE_SEND_PORT);
    let service_recv_port = mm.alloc_port(SERVICE_RECV_PORT);
    let trap_ctx_ppn = mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();
    let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
    *trap_ctx = TrapContext::app_init_context(
        entry_point,
        user_sp,
        KERNEL_SPACE.borrow_mut().token(),
        mm.kernel_stack_top(),
        trap_handler as usize,
    );
    let token = mm.token();
    USER_SPACES.borrow_mut().insert(token, mm);
    (token, service_send_port, service_recv_port)
}

pub fn fork_user_space(token: usize) -> usize {
    let mm = USER_SPACES.borrow_mut().get(&token).unwrap().clone();
    let trap_ctx_ppn = mm.translate(VirtAddr::from(TRAP_CONTEXT).into()).unwrap();
    let trap_ctx: &mut TrapContext = trap_ctx_ppn.get_mut();
    trap_ctx.kernel_sp = mm.kernel_stack_top();
    let token = mm.token();
    USER_SPACES.borrow_mut().insert(token, mm);
    token
}

pub fn remove_user_space(token: usize) {
    log!("[kernel] Remove user space: token = {:x}", token);
    USER_SPACES.borrow_mut().remove(&token).unwrap();
}

pub fn recycle_user_space(token: usize) {
    let mut user_spaces = USER_SPACES.borrow_mut();
    let mm = user_spaces.get_mut(&token).unwrap();
    mm.recycle();
}

pub fn change_program_brk(token: usize, size: i32) -> Option<usize> {
    let mut user_spaces = USER_SPACES.borrow_mut();
    let mm = user_spaces.get_mut(&token).unwrap();
    mm.change_brk(size)
}
