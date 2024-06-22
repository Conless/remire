// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub mod riscv_spec;

mod console;
mod uart;

use crate::config::{MEM_END, MEM_START, SUPERVISOR_ENTRY};

pub(crate) use console::Stdout;
pub(crate) use uart::CONSOLE;

#[inline(always)]
pub fn hart_id() -> usize {
    riscv::register::mhartid::read()
}

pub fn init_uart() {
    uart::console_init();
}

pub fn set_pmp() {
    use riscv::register::*;
    unsafe {
        pmpcfg0::set_pmp(0, Range::OFF, Permission::NONE, false);
        pmpaddr0::write(0);

        pmpcfg0::set_pmp(1, Range::TOR, Permission::RW, false);
        pmpaddr1::write(MEM_START >> 2);

        pmpcfg0::set_pmp(2, Range::TOR, Permission::NONE, false);
        pmpaddr2::write(SUPERVISOR_ENTRY >> 2);

        pmpcfg0::set_pmp(3, Range::TOR, Permission::RWX, false);
        pmpaddr3::write(MEM_END >> 2);

        pmpcfg0::set_pmp(4, Range::TOR, Permission::RW, false);
        pmpaddr4::write(1 << (usize::BITS - 1));
    }
}

pub fn init_bss() {
    extern "C" {
        static mut sbss: u64;
        static mut ebss: u64;
    }
    unsafe {
        let mut ptr = sbss as *mut u64;
        let end = ebss as *mut u64;
        while ptr < end {
            ptr.write_volatile(0);
            ptr = ptr.offset(1);
        }
    }
}
