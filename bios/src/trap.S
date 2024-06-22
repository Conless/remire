.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

# Global trap handler
#
# fn __alltraps(ctx: &mut TrapContext) -> !
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # change sp to machine stack
    csrrw sp, mscratch, sp

    # now sp points to *TrapContext 
    # and sscratch points to user stack
    sd ra, 1*8(sp)
    sd gp, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    csrr t0, mepc
    csrr t1, satp
    sd t0, 32*8(sp) # TrapContext::pc
    sd t1, 33*8(sp) # TrapContext::satp
    
    csrr t2, mscratch
    sd t2, 2*8(sp)  # TrapContext::sp
    
    ld t1, 35*8(sp) # TrapContext::trap_handler
    ld sp, 34*8(sp) # TrapContext::kernel_sp

    # switch to machine mode
    csrw satp, 0
    sfence.vma
    # jump to trap_handler
    jr t1

# Restore from trap
#
# fn __restore(ctx: *mut TrapContext) -> !
__restore:
    csrw mscratch, a0
    mv sp, a0

    # now sp points to TrapContext in user space, start restoring based on it
    # restore mepc/satp
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw mepc, t0
    csrw satp, t1

    # restore registers
    ld ra, 1*8(sp)
    ld gp, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    # back to user stack
    ld sp, 2*8(sp)
    mret
