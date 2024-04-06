// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::panic::PanicInfo;

use crate::exit;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let message = panic_info.message().unwrap();
    eprintln!("Panic: {}", message);
    exit(1);
    unreachable!()
}
