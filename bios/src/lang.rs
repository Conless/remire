// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}