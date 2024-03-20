// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    env, fs, os::unix::process::CommandExt, path::{Path, PathBuf}, process::Command
};

use clap::clap_app;

/// Build mode of the target kernel
enum BuildMode {
    Debug,
    Release,
}

/// Get the root path of the project
fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

/// Convert a command to a string
/// 
/// Since `std::process::Command` does not implement `Display`, we need to implement it
fn command_to_string(command: &std::process::Command) -> String {
    let args = command.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>();
    format!("{} {}", command.get_program().to_string_lossy(), args.join(" "))
}

/// Compile stage
fn compile(mode: &BuildMode) -> bool {
    match mode {
        BuildMode::Debug => {
            println!("[build] Compiling kernel in debug mode");
        }
        BuildMode::Release => {
            println!("[build] Compiling kernel in release mode");
        }
    }
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let ld = project_root().join("target/riscv64gc-unknown-none-elf/linker.ld");
    println!("[build] Writing linker script to {}", ld.to_str().unwrap());
    fs::write(&ld, LINKER).unwrap();
    let mut command = Command::new(cargo);
    command
        .arg("build")
        .arg("--target")
        .arg("riscv64gc-unknown-none-elf");
    if let BuildMode::Release = mode {
        command.arg("--release");
    }
    command.env("RUSTFLAGS", "-C link-arg=-T".to_string() + ld.to_str().unwrap() + " -C force-frame-pointers=yes");
    command.current_dir(project_root());
    println!("[build] Running command: {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute cargo: {}", e);
        return false;
    }
    status.unwrap().success()
}

/// Convert ELF to binary
/// 
/// This function uses `rust-objcopy` to convert the ELF file to a binary file
/// The parameter `mode` is used to determine the path of the Elf file
fn objcopy(mode: &BuildMode) -> bool {
    let kernel_elf = project_root()
        .join("target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("kernel");
    let mut command = Command::new("rust-objcopy");
    command
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .arg(kernel_elf.to_str().unwrap())
        .arg("-O")
        .arg("binary")
        .arg(kernel_elf.with_extension("bin").to_str().unwrap());
    println!("[build] Running command: {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute rust-objcopy: {}", e);
        return false;
    }
    status.unwrap().success()
}

/// Run kernel in QEMU
fn qemu_run(mode: &BuildMode) -> bool {
    let kernel_bin = project_root()
        .join("target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("kernel.bin");
    let mut command = Command::new("qemu-system-riscv64");
    command
        .arg("-nographic")
        .arg("-machine")
        .arg("virt")
        .arg("-bios")
        .arg("none")
        .arg("-device")
        .arg("loader,file=".to_string() + kernel_bin.to_str().unwrap()+ ",addr=0x80000000");
    println!("[run] Running command: {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute qemu-system-riscv64: {}", e);
        return false;
    }
    status.unwrap().success()
}

fn disasm(mode: &BuildMode) -> bool {
    let kernel_elf = project_root()
        .join("target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("kernel");
    let mut command = Command::new("rust-objdump");
    command
        .arg("--arch=riscv64")
        .arg("-D")
        .arg(kernel_elf.to_str().unwrap());
    println!("[disasm] Running command: {}", command_to_string(&command));
    let output = command.output();
    if let Err(e) = output {
        eprintln!("Failed to execute rust-objdump: {}", e);
        return false;
    }
    let tmp_asm = kernel_elf.with_extension("S");
    fs::write(&tmp_asm, output.unwrap().stdout).unwrap();
    let mut vim_command = Command::new("vim");
    vim_command.arg(tmp_asm.to_str().unwrap());
    println!("[disasm] Running command: {}", command_to_string(&vim_command));
    let status = vim_command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute vim: {}", e);
        return false;
    }
    status.unwrap().success()
}

fn main() {
    let matches = clap_app!(xtask =>
        (@subcommand build =>
            (about: "Build project")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand run =>
            (about: "Run kernel in QEMU")
            (@arg release: --release "Run kernel in release mode")
        )
        (@subcommand disasm =>
            (about: "Disassemble kernel")
            (@arg release: --release "Disassemble kernel in release mode")
        )
    )
    .get_matches();

    let mut task_queue: Vec<(&str, Box<dyn Fn(&BuildMode) -> bool>)> = vec![];
    let mut mode: BuildMode = BuildMode::Debug;

    if let Some(matches) = matches.subcommand_matches("build") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("compile", Box::new(compile)));
        task_queue.push(("objcopy", Box::new(objcopy)));
    } else if let Some(matches) = matches.subcommand_matches("run") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("qemu", Box::new(qemu_run)));
    } else if let Some(matches) = matches.subcommand_matches("disasm") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("disasm", Box::new(disasm)));
    }

    for task in task_queue {
        if !task.1(&mode) {
            eprintln!("Execution failed when running task {}", task.0);
        }
    }
}

const LINKER: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80000000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}";
