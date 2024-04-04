// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::panic::PanicInfo;

use crate::println;
use crate::legacy::exit::sbi_shutdown;

#[no_mangle]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // This is a simple panic handler that just prints the panic message to the serial port
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}: {} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    sbi_shutdown()
}
