// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

enum Manager2Kernel {
    SwitchProcess { pid: u8 },
}

enum Kernel2Manager {
    NewProcess { pid: u8, parent_pid: u8 },
    ExitProcess { pid: u8, exit_code: i32 },
}