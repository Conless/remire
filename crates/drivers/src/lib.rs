#![no_std]

pub mod console;
mod uart;

pub fn init_device() {
    console::UART.init();
}
