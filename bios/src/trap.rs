// Copyright (c) 2024 Conless Pan

// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use aclint::SifiveClint as Clint;
use core::arch::{asm, global_asm};
use riscv::register::mcause::{Exception, Trap};
use riscv::register::{mcause, mtval};

use crate::clint::{set_timer, CLINT};
use crate::legacy::console::sbi_console_getchar;
use crate::legacy::exit::sbi_shutdown;
use crate::utils::hart_id;
use crate::{print, println};
use crate::{config::SUPERVISOR_ENTRY, stack::trap_context};

global_asm!(include_str!("trap.S"));

extern "C" {
    fn __alltraps();
    fn __restore();
}
pub struct TrapContext {
    pub regs: [usize; 32],
    pub pc: usize,
    pub satp: usize,
    pub machine_sp: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn init(&mut self, machine_sp: usize) {
        self.pc = SUPERVISOR_ENTRY;
        self.machine_sp = machine_sp;
        self.trap_handler = trap_handler as usize;
    }
}

/// 中断向量表
///
/// # Safety
///
/// 裸函数。
#[naked]
pub(crate) unsafe extern "C" fn trap_vec() {
    asm!(
        ".align 2",
        ".option push",
        ".option norvc",
        "j {default}", // exception
        "j {default}", // supervisor software
        "j {default}", // reserved
        "j {msoft} ",  // machine    software
        "j {default}", // reserved
        "j {default}", // supervisor timer
        "j {default}", // reserved
        "j {mtimer}",  // machine    timer
        "j {default}", // reserved
        "j {default}", // supervisor external
        "j {default}", // reserved
        "j {default}", // machine    external
        ".option pop",
        default = sym __alltraps,
        mtimer  = sym mtimer,
        msoft   = sym msoft,
        options(noreturn)
    )
}

/// machine timer 中断代理
///
/// # Safety
///
/// 裸函数。
#[naked]
#[no_mangle]
unsafe extern "C" fn mtimer() {
    asm!(
        // 换栈：
        // sp      : M sp
        // mscratch: S sp
        "   csrrw sp, mscratch, sp",
        // 保护
        "   addi  sp, sp, -4*8
            sd    ra, 0*8(sp)
            sd    a0, 1*8(sp)
            sd    a1, 2*8(sp)
            sd    a2, 3*8(sp)
        ",
        // 清除 mtimecmp
        "   la    a0, {clint_ptr}
            ld    a0, (a0)
            csrr  a1, mhartid
            addi  a2, zero, -1
            call  {set_mtimecmp}
        ",
        // 设置 stip
        "   li    a0, {mip_stip}
            csrrs zero, mip, a0
        ",
        // 恢复
        "   ld    ra, 0*8(sp)
            ld    a0, 1*8(sp)
            ld    a1, 2*8(sp)
            ld    a2, 3*8(sp)
            addi  sp, sp,  4*8
        ",
        // 换栈：
        // sp      : S sp
        // mscratch: M sp
        "   csrrw sp, mscratch, sp",
        // 返回
        "   mret",
        mip_stip     = const 1 << 5,
        clint_ptr    =   sym CLINT,
        //                   Clint::write_mtimecmp_naked(&self, hart_idx, val)
        set_mtimecmp =   sym Clint::write_mtimecmp_naked,
        options(noreturn)
    )
}

/// machine soft 中断代理
///
/// # Safety
///
/// 裸函数。
#[naked]
#[no_mangle]
unsafe extern "C" fn msoft() {
    asm!(
        // 换栈：
        // sp      : M sp
        // mscratch: S sp
        "   csrrw sp, mscratch, sp",
        // 保护
        "   addi sp, sp, -3*8
            sd   ra, 0*8(sp)
            sd   a0, 1*8(sp)
            sd   a1, 2*8(sp)
        ",
        // 清除 msip 设置 ssip
        "   la   a0, {clint_ptr}
            ld   a0, (a0)
            csrr a1, mhartid
            call {clear_msip}
            csrrsi zero, mip, 1 << 1
        ",
        // 恢复
        "   ld   ra, 0*8(sp)
            ld   a0, 1*8(sp)
            ld   a1, 2*8(sp)
            addi sp, sp,  3*8
        ",
        // 换栈：
        // sp      : S sp
        // mscratch: M sp
        "   csrrw sp, mscratch, sp",
        // 返回
        "   mret",
        clint_ptr  = sym CLINT,
        //               Clint::clear_msip_naked(&self, hart_idx)
        clear_msip = sym Clint::clear_msip_naked,
        options(noreturn)
    )
}

fn trap_handler() -> ! {
    let ctx = trap_context();
    let mcause = mcause::read();
    let mtval = mtval::read();
    match mcause::read().cause() {
        // SBI call
        Trap::Exception(Exception::SupervisorEnvCall) => {
            use sbi_spec::{legacy, time};
            ctx.pc += 4;
            ctx.regs[10] = match ctx.regs[17] {
                legacy::LEGACY_CONSOLE_PUTCHAR => {
                    print!("{}", ctx.regs[10] as u8 as char);
                    0
                }
                legacy::LEGACY_CONSOLE_GETCHAR => {
                    sbi_console_getchar() as usize
                }
                time::EID_TIME => {
                    set_timer(ctx.regs[10] as u64);
                    0
                }
                _ => {
                    panic!("Unsupported SBI call with a7 = {:x}", ctx.regs[17]);
                }
            };
        }
        trap => {
            println!(
                "
-----------------------------
> trap:    {trap:?}
> mstatus: {:#018x}
> mepc:    {:#018x}
> mtval:   {:#018x}
-----------------------------
      ",
                crate::mstatus::read(),
                crate::mepc::read(),
                mtval::read()
            );
            panic!("stopped with unsupported trap")
        }
    }
    trap_return()
}

pub fn trap_return() -> ! {
    let trap_ctx_ptr = trap_context() as *mut TrapContext;
    unsafe {
        asm!(
            "fence.i",
            "j {restore}",
            restore = sym __restore,
            in("a0") trap_ctx_ptr,
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[sbi] hart {} {info}", hart_id());
    println!("[sbi] system shutdown scheduled due to SBI panic");
    sbi_shutdown()
}
