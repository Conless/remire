// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use io::MsgIOQueue;
use task::{Kernel2PM, PM2Kernel};

mod queue;
mod io;

pub mod task;

pub type PM2KernelQueue = MsgIOQueue<Kernel2PM, PM2Kernel, 32, false>;
pub type Kernel2PMQueue = MsgIOQueue<PM2Kernel, Kernel2PM, 32, true>;