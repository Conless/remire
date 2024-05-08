// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::vec;
use alloc::vec::Vec;
use bitflags::*;
use crate::log;

use super::types::*;
use super::frame::{frame_alloc, FrameGuard};

bitflags! {
    /// Page table entry flags
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

/// Page entry of the SV39 page table
///
/// Contains 64 bits, in following format:
/// |  63-54   |53-10|9-8|7|6|5|4|3|2|1|0|
/// | reserved | PPN |RSW|D|A|G|U|X|W|R|V|
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    /// Get the PPN entry
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    /// Get the flags
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    /// Check if the entry is valid
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    /// Check if the entry is readable
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    /// Check if the entry is writable
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    /// Check if the entry is executable
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

/// Structure of the SV39 page table
pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameGuard>,
}

impl Default for PageTable {
    fn default() -> Self {
        PageTable::new()
    }
}

impl PageTable {
    /// Create a new page table
    /// 
    /// Used when create a new memory space
    fn new() -> Self {
        let frame = frame_alloc().expect("[memory] failed to allocate frame");
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    /// Find or create a page table entry by virtual page number
    fn find_create_entry(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idx = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result = None;
        for (i, idx) in idx.iter().enumerate() {
            let entry = &mut ppn.get_pte_array()[*idx];
            if i == 2 { // Leaf node
                result = Some(entry);
                break;
            }
            if !entry.is_valid() { // Not found
                let new_frame = frame_alloc().expect("[memory] failed to allocate frame");
                *entry = PageTableEntry::new(new_frame.ppn, PTEFlags::V);
                self.frames.push(new_frame);
            }
            ppn = entry.ppn();
        }
        result
    }

    /// Find a page table entry by virtual page number
    fn find_entry(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idx = vpn.indexes();
        let mut ppn = self.root_ppn;
        for (i, idx) in idx.iter().enumerate() {
            let entry = &mut ppn.get_pte_array()[*idx];
            if i == 2 { // Leaf node
                return Some(entry);
            }
            if !entry.is_valid() { // Not found
                return None;
            }
            ppn = entry.ppn();
        }
        unreachable!()
    }

    /// Map a virtual page number to a physical page number
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let entry = self.find_create_entry(vpn).expect("[memory] failed to create page table entry");
        if entry.is_valid() {
            panic!("[memory] virtual address {:#x} is already mapped", vpn.0);
        }
        *entry = PageTableEntry::new(ppn, flags | PTEFlags::V);
        log!("[memory] page table {:#x} mapping {:#x} to {:#x}", self.token(), vpn.0, ppn.0);
    }

    /// Unmap a virtual page number
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let entry = self.find_entry(vpn).expect("[memory] failed to find page table entry");
        if !entry.is_valid() {
            panic!("[memory] virtual address {:#x} is not mapped", vpn.0);
        }
        *entry = PageTableEntry::empty();
    }

    /// Find the physical page number of a virtual page number
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_entry(vpn).map(|entry| *entry)
    }
    
    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_entry(va.floor()).map(|entry| {
            let pa: PhysAddr = entry.ppn().into();
            let offset = va.page_offset();
            let pa_usize: usize = pa.into();
            (pa_usize + offset).into()
        })
    }
    
    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }
}

impl From<usize> for PageTable {
    /// Create a page table from a satp value
    /// 
    /// Please ensure the satp value is valid
    fn from(satp: usize) -> Self {
        PageTable {
            root_ppn: PhysPageNum(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }
}

impl From<PageTable> for usize {
    fn from(val: PageTable) -> Self {
        val.root_ppn.0
    }
}
