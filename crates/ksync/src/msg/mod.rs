// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use port::MsgPort;
use task::{Kernel2PM, PM2Kernel};

pub mod queue;
pub mod port;

pub mod task;

pub type PM2KernelPort = MsgPort<Kernel2PM, PM2Kernel, 32, false>;
pub type Kernel2PMPort = MsgPort<PM2Kernel, Kernel2PM, 32, true>;