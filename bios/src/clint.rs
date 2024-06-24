// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

/// The implementation of this file is basically based on rustsbi-qemu

use aclint::SifiveClint;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::{config::CLINT_START, utils::hart_id};

pub(crate) static CLINT: AtomicPtr<SifiveClint> = AtomicPtr::new(CLINT_START as _);

pub fn set_timer(time_value: u64) {
    unsafe {
        riscv::register::mip::clear_stimer();
        (*CLINT.load(Ordering::Relaxed)).write_mtimecmp(hart_id(), time_value);
    }
}

#[inline]
pub fn clear() {
    loop {
        if let Some(clint) = unsafe { CLINT.load(Ordering::Relaxed).as_ref() } {
            clint.clear_msip(hart_id());
            clint.write_mtimecmp(hart_id(), u64::MAX);
            break;
        } else {
            continue;
        }
    }
}
