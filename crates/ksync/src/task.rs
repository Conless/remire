// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{msg::GetID, MsgQueue};

#[derive(Clone, Copy)]
pub enum PM2Kernel {
    ForkReply { id: usize, pid: usize, child_pid: usize },
    Remove { id: usize, token: usize },
    Invalid,
}

impl Default for PM2Kernel {
    fn default() -> Self {
        Self::Invalid
    }
}

impl GetID for PM2Kernel {
    fn get_id(&self) -> usize {
        match self {
            PM2Kernel::ForkReply { id, .. } => *id,
            PM2Kernel::Remove { id, .. } => *id,
            PM2Kernel::Invalid => panic!("Invalid message"),
        }
    }
}

pub type PM2KernelQueue = MsgQueue<PM2Kernel, 32>;

#[derive(Clone, Copy)]
pub enum Kernel2PM {
    Init { id: usize, token: usize },
    Fork { id: usize, pid: usize, token: usize },
    Exec { id: usize, pid: usize, token: usize },
    Exit { id: usize, pid: usize, exit_code: i32 },
    Invalid,
}

impl Default for Kernel2PM {
    fn default() -> Self {
        Self::Invalid
    }
}

impl GetID for Kernel2PM {
    fn get_id(&self) -> usize {
        match self {
            Kernel2PM::Init { id, .. } => *id,
            Kernel2PM::Fork { id, .. } => *id,
            Kernel2PM::Exec { id, .. } => *id,
            Kernel2PM::Exit { id, .. } => *id,
            Kernel2PM::Invalid => panic!("Invalid message"),
        }
    }
}

pub type Kernel2PMQueue = MsgQueue<Kernel2PM, 32>;
