// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::{collections::BTreeMap, vec::Vec};

use crate::config::PAGE_SIZE;

use super::{
    address::{PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    frame::{frame_alloc, FrameGuard}, page_table::{PTEFlags, PageTable}, range::StepByOne,
};
use bitflags::bitflags;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
  pub struct MapPermission: u8 {
      const R = 1 << 1;
      const W = 1 << 2;
      const X = 1 << 3;
      const U = 1 << 4;
  }
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameGuard>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    /// Create a new map area with the given range and map type.
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }

    /// Map a single page.
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => { // Identical mapping does not need to allocate a new frame
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    /// Unmap a single page.
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if let MapType::Framed = self.map_type {
            self.data_frames.remove(&vpn);
        }
        page_table.unmap(vpn);
    }

    /// Map all the pages in the range.
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }
    
    /// Unmap all the pages in the range.
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }

    /// Copy the data into the area.
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        let mut start: usize = 0;
        let mut vpn = self.vpn_range.get_start();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)]; // The data of this range
            let dst = &mut page_table // The destination of this range
                .translate(vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            vpn.step();
        }
    }
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn empty() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    /// Push a mapped area into the memory set.
    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(map_area);
    }

    /// insert a new map area into the memory set.
    pub fn insert(
        &mut self,
        start_va: VirtAddr, end_va: VirtAddr, permission: MapPermission
    ) {
        self.push(MapArea::new( // Note that this range may be conflict with the existing ones
            start_va,
            end_va,
            MapType::Framed,
            permission,
        ), None);
    }
}
