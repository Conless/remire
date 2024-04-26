// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use core::arch::asm;

use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use riscv::register::satp;

use crate::{
    config::{MEMORY_END, MMIO, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT},
    println,
    sync::UPSafeCell,
};

use super::{
    address::{PhysAddr, PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    frame::{frame_alloc, FrameGuard},
    page_table::{PTEFlags, PageTable},
    range::StepByOne,
};
use bitflags::bitflags;
use lazy_static::lazy_static;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

lazy_static! {
    /// a memory set instance through lazy_static! managing kernel space
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(unsafe { UPSafeCell::new(MemorySet::new_kernel()) });
}

pub fn activate_kernel_space() {
    KERNEL_SPACE.borrow_mut().activate();
}

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

/// A mapped area of virtual memory
///
/// `MapArea` represents an area of virtual memory with continuous address in the address space.
pub struct MapArea {
    vpn_range: VPNRange, // The range of the virtual page number, can be traversed by Iter.
    data_frames: BTreeMap<VirtPageNum, FrameGuard>, // The data frames of the area
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    /// Create a new map area with the given range and map type.
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
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
            MapType::Identical => {
                // Identical mapping does not need to allocate a new frame
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

    pub fn shrink_to(&mut self, page_table: &mut PageTable, new_end: VirtPageNum) {
        for vpn in VPNRange::new(new_end, self.vpn_range.get_end()) {
            self.unmap_one(page_table, vpn)
        }
        self.vpn_range = VPNRange::new(self.vpn_range.get_start(), new_end);
    }

    pub fn append_to(&mut self, page_table: &mut PageTable, new_end: VirtPageNum) {
        for vpn in VPNRange::new(self.vpn_range.get_end(), new_end) {
            self.map_one(page_table, vpn)
        }
        self.vpn_range = VPNRange::new(self.vpn_range.get_start(), new_end);
    }
}

/// The memory set of a process
///
/// This struct contains the page table and mapped areas of a process
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
    pub fn token(&self) -> usize {
        self.page_table.token()
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
    pub fn insert(&mut self, start_va: VirtAddr, end_va: VirtAddr, permission: MapPermission) {
        self.push(
            MapArea::new(
                // Note that this range may be conflict with the existing ones
                start_va,
                end_va,
                MapType::Framed,
                permission,
            ),
            None,
        );
    }

    /// Create a kernel address space.
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::empty();

        // map trampoline
        memory_set.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );

        // map kernel sections
        println!(
            "[kernel] mapping .text [{:#x}, {:#x})",
            stext as usize, etext as usize
        );
        memory_set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        println!(
            "[kernel] mapping .rodata [{:#x}, {:#x})",
            srodata as usize, erodata as usize
        );
        memory_set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );
        println!(
            "[kernel] mapping .data [{:#x}, {:#x})",
            sdata as usize, edata as usize
        );
        memory_set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!(
            "[kernel] mapping .bss [{:#x}, {:#x})",
            sbss_with_stack as usize, ebss as usize
        );
        memory_set.push(
            MapArea::new(
                (sbss_with_stack as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        println!(
            "[kernel] mapping physical memory [{:#x}, {:#x})",
            ekernel as usize, MEMORY_END
        );
        memory_set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        for pair in MMIO {
            println!(
                "[kernel] mapping MMIO [{:#x}, {:#x})",
                pair.0,
                pair.0 + pair.1
            );
            memory_set.push(
                MapArea::new(
                    pair.0.into(),
                    (pair.0 + pair.1).into(),
                    MapType::Identical,
                    MapPermission::R | MapPermission::W,
                ),
                None,
            );
        }
        memory_set
    }

    pub fn new_app(app_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::empty();

        // map trampoline
        memory_set.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );

        // map app sections
        let elf_data = xmas_elf::ElfFile::new(app_data).unwrap();
        let elf_header = elf_data.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

        let header_count = elf_header.pt2.ph_count();
        for i in 0..header_count {
            let header = elf_data.program_header(i).unwrap();
            if header.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va = VirtAddr::from(header.virtual_addr() as usize);
                let end_va = VirtAddr::from((header.virtual_addr() + header.mem_size()) as usize);

                // init permission
                let mut permission = MapPermission::U;
                let flags = header.flags();
                if flags.is_read() {
                    permission |= MapPermission::R;
                }
                if flags.is_write() {
                    permission |= MapPermission::W;
                }
                if flags.is_execute() {
                    permission |= MapPermission::X;
                }

                // init mapped area
                let map_area = MapArea::new(start_va, end_va, MapType::Framed, permission);
                memory_set.push(
                    map_area,
                    Some(
                        &app_data[header.offset() as usize
                            ..(header.offset() + header.file_size()) as usize],
                    ),
                );
                println!(
                    "[kernel] mapping app section [{:#x}, {:#x})",
                    usize::from(start_va),
                    usize::from(end_va)
                );
            }
        }

        let end_va: VirtAddr = memory_set.areas.last().unwrap().vpn_range.get_end().into();
        let end_va_usize: usize = end_va.into();
        let user_stack_bottom: usize = end_va_usize + PAGE_SIZE;
        let user_stack_top = user_stack_bottom + PAGE_SIZE;
        println!(
            "[kernel] mapping user stack [{:#x}, {:#x})",
            user_stack_bottom, user_stack_top
        );
        memory_set.push(
            MapArea::new(
                user_stack_bottom.into(),
                user_stack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );

        // TODO: reserve for sbrk (reference: rcore)

        // mapping the trap context
        println!(
            "[kernel] mapping trap context [{:#x}, {:#x})",
            TRAP_CONTEXT, TRAMPOLINE
        );
        memory_set.push(
            MapArea::new(
                TRAP_CONTEXT.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        (
            memory_set,
            user_stack_top,
            elf_data.header.pt2.entry_point() as usize,
        )
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PhysPageNum> {
        self.page_table.translate(vpn).map(|pte| pte.ppn())
    }

    pub fn activate(&self) {
        let satp = self.page_table.token();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }

    pub fn shrink_to(&mut self, start: VirtAddr, new_end: VirtAddr) -> bool {
        if let Some(area) = self
            .areas
            .iter_mut()
            .find(|area| area.vpn_range.get_start() == start.floor())
        {
            area.shrink_to(&mut self.page_table, new_end.ceil());
            true
        } else {
            false
        }
    }

    pub fn append_to(&mut self, start: VirtAddr, new_end: VirtAddr) -> bool {
        if let Some(area) = self
            .areas
            .iter_mut()
            .find(|area| area.vpn_range.get_start() == start.floor())
        {
            area.append_to(&mut self.page_table, new_end.ceil());
            true
        } else {
            false
        }
    }
}
