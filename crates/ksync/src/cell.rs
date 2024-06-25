// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// Uniprocessor interior mutability primitives
// Reference: [rCore](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/3batch-system.html), [interior mutability](https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html)

use core::cell::{RefCell, RefMut};

// A safe cell that can be used in uniprocessor
pub struct UPSafeCell<T>(RefCell<T>);

unsafe impl<T> Sync for UPSafeCell<T> {}
unsafe impl<T> Send for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub const unsafe fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}
