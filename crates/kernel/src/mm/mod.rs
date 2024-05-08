// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod frame;
mod mm_struct;
mod page;
mod page_table;
mod vm_area;

use alloc::{string::String, sync::Arc, vec::Vec};
use lazy_static::lazy_static;

use page::{StepByOne, VirtAddr, VirtPageNum};
use page_table::PageTable;

use crate::sync::UPSafeCell;

pub use frame::init_frame_allocator;
pub use mm_struct::MMStruct;
pub use vm_area::{MapPermission, MapType, VMArea};

pub mod types {
    pub use super::frame::{PhysAddr, PhysPageNum};
    pub use super::page::{Range, StepByOne, VPNRange, VirtAddr, VirtPageNum};
}

lazy_static! {
  /// a memory set instance through lazy_static! managing kernel space
  pub static ref KERNEL_SPACE: Arc<UPSafeCell<MMStruct>> =
      Arc::new(unsafe { UPSafeCell::new(MMStruct::new_kernel()) });
}

pub fn activate_kernel_space() {
    KERNEL_SPACE.borrow_mut().activate();
}

/// translate a pointer to a mutable u8 Vec through page table
pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    let page_table = PageTable::from(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = Vec::new();
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = page_table.translate(vpn).unwrap().ppn();
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        if end_va.page_offset() == 0 {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..]);
        } else {
            v.push(&mut ppn.get_bytes_array()[start_va.page_offset()..end_va.page_offset()]);
        }
        start = end_va.into();
    }
    v
}

pub fn translated_str(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from(token);
    let mut string = String::new();
    let mut va = ptr as usize;
    loop {
        let ch: u8 = *(page_table
            .translate_va(VirtAddr::from(va))
            .unwrap()
            .get_mut());
        if ch == 0 {
            break;
        } else {
            string.push(ch as char);
            va += 1;
        }
    }
    string
}

pub fn translated_ptr<T>(token: usize, ptr: *mut T) -> *mut T {
    let page_table = PageTable::from(token);
    let va = ptr as usize;
    let addr: usize = page_table.translate_va(VirtAddr::from(va)).unwrap().into();
    addr as *mut T
}
