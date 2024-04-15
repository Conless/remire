// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use crate::{println, task::exit_to_next};

/// Exit the current application
///
/// This function will print the exit code of the application and run the next application.
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_to_next();
    unreachable!()
}
