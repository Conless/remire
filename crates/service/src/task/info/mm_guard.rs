// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use ksync::msg::task::PM2Kernel;

use crate::msg::{send_msg};

pub struct MMGuard(pub usize);

impl MMGuard {
    pub fn from_token(token: usize) -> Self {
        log!("MMGuard from token: {:x}", token);
        MMGuard(token)
    }
}

impl Drop for MMGuard {
    fn drop(&mut self) {
        log!("[kernel] Drop MMGuard: {:x}", self.0);
        send_msg(PM2Kernel::Remove { token: self.0 })
    }
}
