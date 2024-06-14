// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sched::proc::current_pid;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100; // Interrupts every 10 ms
const MSEC_PER_SEC: usize = 1000; // Milliseconds per second

pub fn get_time() -> usize {
    time::read()
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn set_next_interrupt() {
    if current_pid() != 0 {
        set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
    } else {
        set_timer(usize::MAX)
    }
}
