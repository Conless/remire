// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use crate::config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT};

pub fn get_num_app() -> usize {
  extern "C" {
      fn _num_app();
  }
  unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_addr(app_id: usize) -> usize {
  APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

fn clear_app_region(app_id: usize) {
  let addr = get_app_addr(app_id);
  (addr..addr+APP_SIZE_LIMIT).for_each(|addr| unsafe {
    (addr as *mut u8).write_volatile(0)
  });
}

pub fn load_apps() {
  extern "C" { fn _num_app(); }
  let num_app_ptr = _num_app as usize as *const usize;
  let num_app = get_num_app();
  let app_start = unsafe {
    core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1)
  };

  for i in 0..num_app {
    let addr = get_app_addr(i);
    clear_app_region(i);
    let src = unsafe {
      core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i+1] - app_start[i])
    };
    let dst = unsafe {
      core::slice::from_raw_parts_mut(addr as *mut u8, src.len())
    };
    dst.copy_from_slice(src);
  }

  unsafe {
    asm!("fence.i");
  }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}