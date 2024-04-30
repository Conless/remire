// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{frame_dealloc, PhysPageNum};

pub struct FrameGuard {
    pub ppn: PhysPageNum,
}

impl FrameGuard {
    /// Create a new frame guard and clear the frame
    pub fn new(ppn: PhysPageNum) -> Self {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}
