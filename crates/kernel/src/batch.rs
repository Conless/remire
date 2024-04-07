// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use crate::{config::*, println, sbi::shutdown, stack::{KERNEL_STACK, USER_STACK}, sync::UPSafeCell, trap::TrapContext};
use lazy_static::lazy_static;

struct AppManager {
    app_count: usize,
    current_app: usize,
    app_addr: [usize; APP_MAX_NUM + 1],
}

impl AppManager {
    pub fn print_info(&self) {
        println!("[kernel] num_app = {}", self.app_count);
        for i in 0..self.app_count {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_addr[i],
                self.app_addr[i + 1]
            );
        }
    }

    pub unsafe fn run(&mut self) {
        if self.current_app >= self.app_count {
            println!("All applications completed!");
            shutdown(false);
        }
        println!("[kernel] Loading app_{}", self.current_app);
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_addr[self.current_app] as *const u8,
            self.app_addr[self.current_app + 1] - self.app_addr[self.current_app],
        );
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
        asm!("fence.i");
        self.current_app += 1;
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
            USER_STACK.get_sp(),
        )) as *const TrapContext as usize);
    }
    unreachable!()
}


