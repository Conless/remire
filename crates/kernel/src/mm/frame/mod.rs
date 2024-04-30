// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

mod manager;
mod guard;
mod types;

pub use types::{PhysAddr, PhysPageNum};
pub use guard::FrameGuard;
pub use manager::{frame_alloc, frame_dealloc, init_frame_allocator};


