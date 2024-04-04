// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::sync::atomic::{AtomicU8, Ordering};

pub(crate) static CONSOLE: Uart16550Map = Uart16550Map(0x1000_0000 as _);

pub(crate) fn console_init() {
    CONSOLE.get().init();
}

use bitflags::bitflags;

bitflags! {
    /// Interrupt enable flags.
    pub struct IntEnFlags: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGED = 1 << 3;
    }
}

bitflags! {
    /// Line status flags
    struct LineStatusFlags: u8 {
        // TODO: ZCore says the other bits are unknown
        const INPUT_FULL = 1;
        const OUTPUT_EMPTY = 1 << 5;
    }
}

#[repr(C)]
pub struct Uart16550<T> {
    /// Data register: read to receive, write to send.
    data: T,
    /// Interrupt enable register.
    int_en: T,
    /// FIFO control register.
    fifo_ctrl: T,
    /// Line control register.
    line_ctrl: T,
    /// Modem control register.
    modem_ctrl: T,
    /// Line status register.
    line_status: T,
    /// Modem status register.
    modem_status: T,
}

impl Uart16550<AtomicU8> {
    /// Initialize UART
    pub fn init(&self) {
        // self.int_en.store(0x00, Ordering::Relaxed);
        // self.fifo_ctrl.store(0xC7, Ordering::Relaxed);
        // self.line_ctrl.store(0x0B, Ordering::Relaxed);
        // self.int_en.store(0x01, Ordering::Relaxed);
    }

    /// Get line status.
    fn line_sts(&self) -> LineStatusFlags {
         LineStatusFlags::from_bits_truncate(self.line_status.load(Ordering::Relaxed))
    }

    pub fn read(&self) -> u8 {
        while !self.line_sts().contains(LineStatusFlags::INPUT_FULL) {}
        self.data.load(Ordering::Relaxed)
    }

    pub fn write(&self, ch: u8) {
        while !self.line_sts().contains(LineStatusFlags::OUTPUT_EMPTY) {}
        self.data.store(ch, Ordering::Relaxed);
    }
}

pub struct Uart16550Map(*const Uart16550<AtomicU8>);

unsafe impl Send for Uart16550Map {}
unsafe impl Sync for Uart16550Map {}

impl Uart16550Map {
    #[inline]
    pub fn get(&self) -> &Uart16550<AtomicU8> {
        unsafe { &*self.0 }
    }
}
