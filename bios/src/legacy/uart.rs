// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

/// The implementation of this file references [zCore](https://github.com/rcore-os/zCore) and [recore](https://github.com/Celve/recore)

use core::{
    fmt::Error,
    sync::atomic::{AtomicPtr, Ordering},
};

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

impl Uart16550<AtomicPtr<u8>> {
    // Create an Uart instance with the given base address
    pub unsafe fn new(base: usize) -> Self {
        let base_pointer = base as *mut u8;
        Self {
            data: AtomicPtr::new(base_pointer),
            int_en: AtomicPtr::new(base_pointer.add(1)),
            fifo_ctrl: AtomicPtr::new(base_pointer.add(2)),
            line_ctrl: AtomicPtr::new(base_pointer.add(3)),
            modem_ctrl: AtomicPtr::new(base_pointer.add(4)),
            line_status: AtomicPtr::new(base_pointer.add(5)),
            modem_status: AtomicPtr::new(base_pointer.add(6)),
        }
    }

    /// Initialize UART
    pub fn init(&self) {
        // let self_data = self.data.load(Ordering::Relaxed);
        let self_int_en = self.int_en.load(Ordering::Relaxed);
        let self_fifo_ctrl = self.fifo_ctrl.load(Ordering::Relaxed);
        // let self_line_ctrl = self.line_ctrl.load(Ordering::Relaxed);
        let self_modem_ctrl = self.modem_ctrl.load(Ordering::Relaxed);

        unsafe {
            // Disable interrupts
            self_int_en.write(0x00.into());

            // Enable FIFO, clear TX/RX queues and
            // set interrupt watermark at 14 bytes
            self_fifo_ctrl.write(0xC7.into());

            // Mark data terminal ready, signal request to send
            // and enable auxilliary output #2 (used as interrupt line for CPU)
            self_modem_ctrl.write(0x0B.into());

            // Enable interrupts
            self_int_en.write(0x01.into());
        }
    }

    /// Get line status.
    fn line_sts(&self) -> LineStatusFlags {
        unsafe { LineStatusFlags::from_bits_truncate(*self.line_status.load(Ordering::Relaxed)) }
    }

    fn try_recv(&self) -> Result<Option<u8>, Error> {
        if self.line_sts().contains(LineStatusFlags::INPUT_FULL) {
            let data = unsafe { self.data.load(Ordering::Relaxed).read() };
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    /// Receive a byte from UART.
    pub fn recv(&self) -> Result<u8, Error> {
        loop {
            if let Some(data) = self.try_recv()? {
                return Ok(data);
            }
        }
    }

    pub fn send(&self, ch: u8) -> Result<(), Error> {
        while !self.line_sts().contains(LineStatusFlags::OUTPUT_EMPTY) {}
        unsafe { self.data.load(Ordering::Relaxed).write(ch.into()) };
        Ok(())
    }
}
