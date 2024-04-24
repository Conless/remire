// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::{alloc::Layout, clone, cmp::{max, min}, mem::size_of};

use crate::println;

use super::avl::AVLTree;

const BUDDY_ALLOCATOR_LEVEL: usize = 32;

pub struct BuddyAllocator {
    // Unallocated memory
    free_list: [AVLTree; BUDDY_ALLOCATOR_LEVEL],

    // Statistics
    user: usize,
    allocated: usize,
    total: usize,
}

impl BuddyAllocator {
    /// Helper function for splitting into lower layers
    fn split(&mut self, from: usize, to: usize) {
        for i in (to+1..from+1).rev() {
            if let Some(block) = self.free_list[i].pop_min() {
                self.free_list[i - 1].insert(block);
                self.free_list[i - 1].insert(block + (1 << (i - 1)));
                // println!("[allocator] split: {:#x} -> {:#x}", i, i-1);
            } else {
                panic!("[allocator] internal error: buddy allocator is corrupted");
            }
        }
    }

    /// Helper function for merging into higher layers
    fn merge(&mut self, from: usize, ptr: usize) {
        let mut layer = from;
        let mut key = ptr;
        while layer < self.free_list.len() {
            let buddy = key ^ (1 << layer);
            if self.free_list[layer].delete(buddy) {
                key = min(key, buddy);
                layer += 1;
            } else {
                self.free_list[layer].insert(key);
                break;
            }
        }
    }
}

impl BuddyAllocator {
    pub const fn empty() -> Self {
        Self {
            free_list: [AVLTree::new(); BUDDY_ALLOCATOR_LEVEL],
            user: 0,
            allocated: 0,
            total: 0,
        }
    }

    pub unsafe fn add_segment(&mut self, mut start: usize, mut end: usize) {
        let align = 32;
        start = (start + align - 1) & !(align - 1);
        end &= !(align - 1);
        self.total += end - start;

        while start < end {
            let level = (end - start).trailing_zeros() as usize;
            self.free_list[level].insert(start);
            start += 1 << level;
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), 32),
        );
        let level = size.trailing_zeros() as usize;
        for i in level..self.free_list.len() {
            // Find the first non-empty layer
            if !self.free_list[i].is_empty() {
                self.split(i, level);
                let result = self.free_list[level].pop_min().expect("[allocator] Expect non-empty free list.");

                // Maintain statistics
                // println!("[allocator] allocated: {:#x}, user: {:#x}", self.allocated, self.user);
                self.user += size;
                self.allocated += size;
                return result as *mut u8;
            }
        }
        panic!("[allocator] out of memory")
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), 32),
        );
        let level = size.trailing_zeros() as usize;
        self.free_list[level].insert(ptr as usize);
        self.user -= size;
    }
}
