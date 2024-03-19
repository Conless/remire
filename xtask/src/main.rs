use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use clap::clap_app;

enum BuildMode {
    Debug,
    Release,
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn command_to_string(command: &std::process::Command) -> String {
    let args = command.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>();
    format!("{} {}", command.get_program().to_string_lossy(), args.join(" "))
}

fn compile(mode: &BuildMode) -> bool {
    match mode {
        BuildMode::Debug => {
            println!("Compiling kernel in debug mode");
        }
        BuildMode::Release => {
            println!("Compiling kernel in release mode");
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
    command.current_dir(project_root());
    println!("[build] {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute cargo: {}", e);
        return false;
    }
    status.unwrap().success()
}

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
    println!("[build] {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute rust-objcopy: {}", e);
        return false;
    }
    status.unwrap().success()
}

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
        .arg("loader,file=".to_string() + kernel_bin.to_str().unwrap());
    println!("[run] {}", command_to_string(&command));
    let status = command.status();
    if let Err(e) = status {
        eprintln!("Failed to execute qemu-system-riscv64: {}", e);
        return false;
    }
    status.unwrap().success()
}

fn main() {
    let matches = clap_app!(xtask =>
        (@subcommand make =>
            (about: "Build project")
            (@arg release: --release "Build artifacts in release mode, with optimizations")
        )
        (@subcommand qemu =>
            (about: "Run kernel in QEMU")
            (@arg release: --release "Run kernel in release mode")
        )
    )
    .get_matches();

    let mut task_queue: Vec<(&str, Box<dyn Fn(&BuildMode) -> bool>)> = vec![];
    let mut mode: BuildMode = BuildMode::Debug;

    if let Some(matches) = matches.subcommand_matches("make") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("compile", Box::new(compile)));
        task_queue.push(("objcopy", Box::new(objcopy)));
    } else if let Some(matches) = matches.subcommand_matches("qemu") {
        if matches.is_present("release") {
            mode = BuildMode::Release;
        }
        task_queue.push(("qemu", Box::new(qemu_run)));
    }

    for task in task_queue {
        if !task.1(&mode) {
            eprintln!("Execution failed when running task {}", task.0);
        }
    }
}
