// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use crate::{config::*, stack::KERNEL_STACK, sync::UPSafeCell, trap::TrapContext};
use lazy_static::lazy_static;

struct AppManager {
    app_count: usize,
    current_app: usize,
    app_addr: [usize; APP_MAX_NUM + 1],
}

impl AppManager {
    unsafe fn run(&mut self) {
        self.current_app += 1;
        if self.current_app >= self.app_count {
            panic!("[kernel] No more application to run!");
        }
        let app_addr = self.app_addr[self.current_app];
        let app_src = 
            core::slice::from_raw_parts(
                app_addr as *const u8,
                self.app_addr[self.current_app + 1] - app_addr,
            )
        ;
        let app_dst =
             core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len()) ;
        app_dst.copy_from_slice(app_src);
        asm!("fence.i");
    }
}

lazy_static! {
  static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
      UPSafeCell::new({
          extern "C" {
              fn _num_app();
          }
          let app_ptr = _num_app as usize as *const usize;
          let app_count = app_ptr.read_volatile();
          let mut app_addr: [usize; APP_MAX_NUM + 1] = [0; APP_MAX_NUM + 1];
          let app_start_raw: &[usize] =
              core::slice::from_raw_parts(app_ptr.add(1), app_count + 1);
          app_addr[..=app_count].copy_from_slice(app_start_raw);
          AppManager {
              app_count,
              current_app: 0,
              app_addr,
          }
      })
  };
}

pub fn run_next_app() -> ! {
    unsafe {
        APP_MANAGER.borrow_mut().run();
    }
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            APP_SIZE_LIMIT,
        )) as *const TrapContext as usize);
    }
    unreachable!()
}


