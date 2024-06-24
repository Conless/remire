// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![no_std]
#![no_main]

use service::msg::recv_msg;

extern crate service;

#[no_mangle]
pub fn main() -> i32 {
  loop {
    let (id, msg) = recv_msg(0);
    
  }
}