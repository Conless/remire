// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use alloc::vec::Vec;
use core::arch::asm;
use riscv::register::satp;

use super::frame::{PhysAddr, PhysPageNum};
use super::page::{VirtAddr, VirtPageNum};
use super::page_table::{PTEFlags, PageTable};
use super::vm_area::{MapPermission, MapType, VMArea};

use crate::{config::{MEMORY_END, MMIO, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT}, println};

/// The memory set of a process
///
/// This struct contains the page table and mapped areas of a process
pub struct MMStruct {
  page_table: PageTable,
  areas: Vec<VMArea>,
}


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

impl MMStruct {
  pub fn empty() -> Self {
      Self {
          page_table: PageTable::default(),
          areas: Vec::new(),
      }
  }
  pub fn token(&self) -> usize {
      self.page_table.token()
  }

  /// Push a mapped area into the memory set.
  fn push(&mut self, mut map_area: VMArea, data: Option<&[u8]>) {
      map_area.map(&mut self.page_table);
      if let Some(data) = data {
          map_area.copy_data(&mut self.page_table, data);
      }
      self.areas.push(map_area);
  }

  /// insert a new map area into the memory set.
  pub fn insert(&mut self, start_va: VirtAddr, end_va: VirtAddr, permission: MapPermission) {
      self.push(
        VMArea::new(
              // Note that this range may be conflict with the existing ones
              start_va,
              end_va,
              MapType::Framed,
              permission,
          ),
          None,
      );
  }
  
  pub fn remove(&mut self, start_vpn: VirtPageNum) {
      if let Some((idx, area)) = self
          .areas
          .iter_mut()
          .enumerate()
          .find(|(_, area)| area.get_start() == start_vpn)
      {
          area.unmap(&mut self.page_table);
          self.areas.remove(idx);
      }
  }

  /// Create a kernel address space.
  pub fn new_kernel() -> Self {
      let mut memory_set = Self::empty();

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
          VMArea::new(
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
        VMArea::new(
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
        VMArea::new(
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
        VMArea::new(
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
        VMArea::new(
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
            VMArea::new(
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
              let map_area = VMArea::new(start_va, end_va, MapType::Framed, permission);
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

      let end_va: VirtAddr = memory_set.areas.last().unwrap().get_end().into();
      let end_va_usize: usize = end_va.into();
      let user_stack_bottom: usize = end_va_usize + PAGE_SIZE;
      let user_stack_top = user_stack_bottom + PAGE_SIZE;
      println!(
          "[kernel] mapping user stack [{:#x}, {:#x})",
          user_stack_bottom, user_stack_top
      );
      memory_set.push(
        VMArea::new(
              user_stack_bottom.into(),
              user_stack_top.into(),
              MapType::Framed,
              MapPermission::R | MapPermission::W | MapPermission::U,
          ),
          None,
      );

      // mapping user heap
      println!(
          "[kernel] mapping user heap [{:#x}, {:#x})",
          user_stack_top, user_stack_top
      );
      memory_set.push(
        VMArea::new(
              user_stack_top.into(),
              user_stack_top.into(),
              MapType::Framed,
              MapPermission::R | MapPermission::W | MapPermission::U,
          ),
          None,
      );

      // mapping the trap context
      println!(
          "[kernel] mapping trap context [{:#x}, {:#x})",
          TRAP_CONTEXT, TRAMPOLINE
      );
      memory_set.push(
        VMArea::new(
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
          .find(|area| area.get_start() == start.floor())
      {
          area.shrink_to(&mut self.page_table, new_end.ceil());
          true
      } else {
          false
      }
  }

  pub fn append_to(&mut self, start: VirtAddr, new_end: VirtAddr) -> bool {
      for area in &self.areas {
          println!("start = {:#x}, end = {:#x}", area.get_start().0, area.get_end().0);
      }
      println!("start = {:#x}, end = {:#x}", start.0, new_end.0);
      // panic!();
      if let Some(area) = self
          .areas
          .iter_mut()
          .find(|area| area.get_start() == start.floor())
      {
          area.append_to(&mut self.page_table, new_end.ceil());
          true
      } else {
          false
      }
  }
}
