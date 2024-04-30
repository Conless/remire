// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::range::{Range, StepByOne};

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

/// Virtual address
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

/// Virtual page number
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

pub type VPNRange = Range<VirtPageNum>;

impl From<usize> for VirtAddr {
  fn from(v: usize) -> Self {
      Self(v & ((1 << VA_WIDTH_SV39) - 1))
  }
}
impl From<usize> for VirtPageNum {
  fn from(v: usize) -> Self {
      Self(v & ((1 << VPN_WIDTH_SV39) - 1))
  }
}


impl From<VirtAddr> for usize {
  fn from(v: VirtAddr) -> Self {
      if v.0 >= (1 << (VA_WIDTH_SV39 - 1)) {
          v.0 | (!((1 << VA_WIDTH_SV39) - 1))
      } else {
          v.0
      }
  }
}
impl From<VirtPageNum> for usize {
  fn from(v: VirtPageNum) -> Self {
      v.0
  }
}

impl VirtAddr {
  /// Align the virtual address to the last virtual page number
  pub fn floor(&self) -> VirtPageNum {
      VirtPageNum(self.0 / PAGE_SIZE)
  }

  /// Align the virtual address to the next virtual page number
  pub fn ceil(&self) -> VirtPageNum {
      if self.0 == 0 {
          VirtPageNum(0)
      } else {
          VirtPageNum((self.0 - 1 + PAGE_SIZE) / PAGE_SIZE)
      }
  }

  /// Get the offset of the virtual address in the page
  pub fn page_offset(&self) -> usize {
      self.0 & (PAGE_SIZE - 1)
  }

  /// Check if the virtual address is aligned
  pub fn aligned(&self) -> bool {
      self.page_offset() == 0
  }
}

impl From<VirtAddr> for VirtPageNum {
  fn from(v: VirtAddr) -> Self {
      assert_eq!(v.page_offset(), 0);
      v.floor()
  }
}
impl From<VirtPageNum> for VirtAddr {
  fn from(v: VirtPageNum) -> Self {
      Self(v.0 << PAGE_SIZE_BITS)
  }
}

impl VirtPageNum {
  pub fn indexes(&self) -> [usize; 3] {
      let mut vpn = self.0;
      let mut idx = [0usize; 3];
      for i in (0..3).rev() {
          idx[i] = vpn & 511;
          vpn >>= 9;
      }
      idx
  }
}
impl StepByOne for VirtPageNum {
  fn step(&mut self) {
      self.0 += 1;
  }

}
