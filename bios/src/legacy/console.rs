// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::sync::atomic::AtomicPtr;
use spin::lock_api::Mutex;

use super::uart::Uart16550;

struct UartWrapper(*const Uart16550<AtomicPtr<u8>>);

unsafe impl Send for UartWrapper {}
unsafe impl Sync for UartWrapper {}

impl UartWrapper {
    #[inline]
    pub fn get(&self) -> &Uart16550<AtomicPtr<u8>> {
        unsafe { &*self.0 }
    }
}

static mut CONSOLE: Mutex<UartWrapper> = Mutex::new(UartWrapper(core::ptr::null()));

pub(crate) fn console_init() {
    unsafe {
        CONSOLE.lock().0 = &Uart16550::new(0x1000_0000);
        CONSOLE.get_mut().get().init();
    }
}

pub(crate) fn sbi_console_putchar(c: u8) -> i32 {
    let result = unsafe {
        CONSOLE.get_mut().get().send(c)
    };
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub(crate) fn sbi_console_getchar() -> i32 {
    let result = unsafe {
        CONSOLE.get_mut().get().recv()
    };
    match result {
        Ok(c) => c as i32,
        Err(_) => -1,
    }
}
