// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod console;
mod device;
mod hsm_cell;
mod riscv_spec;

pub(crate) use console::{Stdin, Stdout};
pub(crate) use device::{BoardInfo, parse};
pub(crate) use hsm_cell::{HsmCell, LocalHsmCell, RemoteHsmCell};
pub(crate) use riscv_spec::*;

use crate::constants::*;

#[inline(always)]
pub fn hart_id() -> usize {
    riscv::register::mhartid::read()
}

pub fn set_pmp(board_info: &BoardInfo) {
    use riscv::register::*;
    let mem = &board_info.mem;
    unsafe {
        pmpcfg0::set_pmp(0, Range::OFF, Permission::NONE, false);
        pmpaddr0::write(0);

        pmpcfg0::set_pmp(1, Range::TOR, Permission::RW, false);
        pmpaddr1::write(mem.start >> 2);

        pmpcfg0::set_pmp(2, Range::TOR, Permission::NONE, false);
        pmpaddr2::write(OS_ENTRY_ADDR >> 2);

        pmpcfg0::set_pmp(3, Range::TOR, Permission::RWX, false);
        pmpaddr3::write(mem.end >> 2);

        pmpcfg0::set_pmp(4, Range::TOR, Permission::RW, false);
        pmpaddr4::write(1 << (usize::BITS - 1));
    }
}