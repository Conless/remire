// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#[derive(Clone, Copy)]
pub enum PM2Kernel {
    ForkReply { child_pid: usize },
    WaitPIDReply { result: isize, exit_code: i32 },
    Recycle { token: usize },
    Remove { token: usize },
    Invalid,
}

impl Default for PM2Kernel {
    fn default() -> Self {
        Self::Invalid
    }
}

#[derive(Clone, Copy)]
pub enum Kernel2PM {
    Init { token: usize },
    Fork { pid: usize, token: usize },
    Exec { pid: usize, token: usize },
    WaitPID { pid: usize, child_pid: isize },
    Exit { pid: usize, exit_code: i32 },
    Invalid,
}

impl Default for Kernel2PM {
    fn default() -> Self {
        Self::Invalid
    }
}
