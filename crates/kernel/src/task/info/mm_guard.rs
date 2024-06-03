// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    mm::{new_user_space, remove_user_space},
    loader::get_app_data_by_name,
};

pub struct MMGuard(pub usize);

impl MMGuard {
    pub fn from_name(app_name: &str) -> Option<Self> {
        if let Some(data) = get_app_data_by_name(app_name) {
            Some(MMGuard(new_user_space(data)))
        } else {
            None
        }
    }
    
    pub fn from_token(token: usize) -> Self {
        MMGuard(token)
    }
    
    pub fn from_elf(elf_data: &[u8]) -> Self {
        MMGuard(new_user_space(elf_data))
    }
}

impl Drop for MMGuard {
    fn drop(&mut self) {
        remove_user_space(self.0)
    }
}
