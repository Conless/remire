// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::io::Write;
use std::{
    env,
    fs::{self, read_dir, File},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
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
    let args = command
        .get_args()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>();
    format!(
        "{} {}",
        command.get_program().to_string_lossy(),
        args.join(" ")
    )
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
    let dir = project_root().join("target/riscv64gc-unknown-none-elf");
    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }
    let ld = project_root().join("target/riscv64gc-unknown-none-elf/linker.ld");
    println!("[build] Writing linker script to {}", ld.to_str().unwrap());
    fs::write(&ld, LINKER).unwrap();
    println!("[build] Writing app data to crates/kernel/src/link_app.S");
    insert_app_data().unwrap();
    let mut command = Command::new(cargo);
    command
        .arg("build")
        .arg("--target")
        .arg("riscv64gc-unknown-none-elf");
    if let BuildMode::Release = mode {
        command.arg("--release");
    }
    command.env(
        "RUSTFLAGS",
        "-C link-arg=-T".to_string() + ld.to_str().unwrap() + " -C force-frame-pointers=yes",
    );
    command.current_dir(project_root());
    println!("[build] Running command: {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute cargo: {}", e);
        return false;
    }
    status.unwrap().success()
}

fn compile_bios(mode: &BuildMode) -> bool {
    match mode {
        BuildMode::Debug => {
            println!("[build] Compiling BIOS in debug mode");
        }
        BuildMode::Release => {
            println!("[build] Compiling BIOS in release mode");
        }
    }
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command
        .arg("build")
        .arg("--target")
        .arg("riscv64gc-unknown-none-elf");
    if let BuildMode::Release = mode {
        command.arg("--release");
    }
    command.current_dir(project_root().join("bios"));
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
    let bios_elf = project_root()
        .join("bios/target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("bios");
    let mut command = Command::new("rust-objcopy");
    command
        .arg("--binary-architecture=riscv64")
        .arg("--strip-all")
        .arg(bios_elf.to_str().unwrap())
        .arg("-O")
        .arg("binary")
        .arg(bios_elf.with_extension("bin").to_str().unwrap());
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
    let bios_bin = project_root()
        .join("bios/target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("rustsbi-qemu.bin");
    let mut command = Command::new("qemu-system-riscv64");
    command
        .arg("-nographic")
        .arg("-machine")
        .arg("virt")
        .arg("-bios")
        .arg(bios_bin.to_str().unwrap())
        .arg("-device")
        .arg("loader,file=".to_string() + kernel_bin.to_str().unwrap() + ",addr=0x80200000");
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
    println!(
        "[disasm] Running command: {}",
        command_to_string(&vim_command)
    );
    let status = vim_command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute vim: {}", e);
        return false;
    }
    status.unwrap().success()
}

fn qemu_debug(mode: &BuildMode) -> bool {
    let kernel_bin = project_root()
        .join("target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("kernel.bin");
    let bios_bin = project_root()
        .join("bios/target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("rustsbi-qemu.bin");
    let mut command = Command::new("qemu-system-riscv64");
    command
        .arg("-nographic")
        .arg("-machine")
        .arg("virt")
        .arg("-bios")
        .arg(bios_bin.to_str().unwrap())
        .arg("-device")
        .arg("loader,file=".to_string() + kernel_bin.to_str().unwrap() + ",addr=0x80200000")
        .arg("-s")
        .arg("-S");
    println!("[run] Running command: {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute qemu-system-riscv64: {}", e);
        return false;
    }
    status.unwrap().success()
}

fn gdb(mode: &BuildMode) -> bool {
    let kernel_elf = project_root()
        .join("target/riscv64gc-unknown-none-elf")
        .join(if let BuildMode::Release = mode {
            "release"
        } else {
            "debug"
        })
        .join("kernel");
    let mut command = Command::new("riscv64-elf-gdb");
    command
        .arg("-ex")
        .arg("file ".to_string() + kernel_elf.to_str().unwrap())
        .arg("-ex")
        .arg("set arch riscv:rv64")
        .arg("-ex")
        .arg("target remote :1234");
    println!("[gdb] Running command: {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute riscv64-elf-gdb: {}", e);
        return false;
    }
    status.unwrap().success()
}

static TARGET_PATH: &str = "crates/user/target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<(), std::io::Error> {
    let mut f = File::create(project_root().join("crates/kernel/src/link_app.S")).unwrap();
    let mut apps: Vec<_> = read_dir(project_root().join("crates/user/src/bin"))
        .unwrap()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, project_root().join(TARGET_PATH).to_str().unwrap()
        )?;
    }
    Ok(())
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
        (@subcommand debug =>
            (about: "Run kernel in QEMU with GDB")
            (@arg release: --release "Run kernel in release mode")
        )
        (@subcommand gdb =>
            (about: "Run GDB")
            (@arg release: --release "Run GDB in release mode")
        )
    )
    .get_matches();

    type TaskFunc = fn(&BuildMode) -> bool;
    let mut task_queue: Vec<(&str, Box<TaskFunc>)> = vec![];
    let mut mode: BuildMode = BuildMode::Debug;

    if let Some(matches) = matches.subcommand_matches("build") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("compile", Box::new(compile)));
        task_queue.push(("compile_bios", Box::new(compile_bios)));
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
    } else if let Some(matches) = matches.subcommand_matches("debug") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("qemu_debug", Box::new(qemu_debug)));
    } else if let Some(matches) = matches.subcommand_matches("gdb") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("gdb", Box::new(gdb)));
    }

    for task in task_queue {
        if !task.1(&mode) {
            eprintln!("Execution failed when running task {}", task.0);
            std::process::exit(1);
        }
    }
}

const LINKER: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

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
