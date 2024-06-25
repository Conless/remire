// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#[derive(Clone, Copy)]
pub enum Kernel2FS {
    Open { pid: usize, path: usize, len: usize, flags: usize, mode: usize },
    Write { pid: usize, fd: usize, buf: usize, len: usize },
    Read { pid: usize, fd: usize, buf: usize, len: usize },
    Close { pid: usize, fd: usize },
    Exit { pid: usize },
    Fork { pid: usize, child_pid: usize },
    Exec { pid: usize, path: usize, len: usize },
    Invalid,
}

impl Default for FS2Kernel {
    fn default() -> Self {
        Self::Invalid
    }
}

#[derive(Clone, Copy)]
pub enum FS2Kernel {
    OpenReply { result: isize },
    WriteReply { result: isize },
    ReadReply { result: isize },
    CloseReply { result: isize },
    ExecReply { dest: usize, len: usize },
    Invalid
}

impl Default for Kernel2FS {
    fn default() -> Self {
        Self::Invalid
    }
}
