// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    loader::get_app_data_by_name, log, mm::{new_user_space, remove_user_space}
};

pub struct MMGuard(pub usize);

impl MMGuard {
    pub fn from_name(app_name: &str) -> Option<Self> {
        let result = get_app_data_by_name(app_name).map(|data| MMGuard(new_user_space(data)));
        match &result {
            Some(guard) => log!("MMGuard from name: {} -> {:x}", app_name, guard.0),
            None => log!("MMGuard from name: {} -> None", app_name),
        }
        result
    }
    
    pub fn from_token(token: usize) -> Self {
        log!("MMGuard from token: {:x}", token);
        MMGuard(token)
    }
}

impl Drop for MMGuard {
    fn drop(&mut self) {
        log!("[kernel] Drop MMGuard: {:x}", self.0);
        remove_user_space(self.0)
    }
}
