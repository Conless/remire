// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod frame;
mod page;
mod vm_area;
mod page_table;
mod mm_struct;

use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;

use page::{StepByOne, VirtAddr};
use page_table::PageTable;

use crate::sync::UPSafeCell;

pub use mm_struct::MMStruct;
pub use vm_area::{VMArea, MapPermission, MapType};
pub use frame::init_frame_allocator;

pub mod types {
  pub use super::frame::{PhysAddr, PhysPageNum};
  pub use super::page::{VirtAddr, VirtPageNum, VPNRange, Range, StepByOne};
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

