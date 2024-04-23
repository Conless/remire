# __switch(
#     current_task_cx_ptr: *mut TaskContext,
#     next_task_cx_ptr: *const TaskContext
# )

.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

# Switch to the next task and return the sp of the next task
#
# fn __switch(
#     current_task_cx_ptr: *mut TaskContext,
#     next_task_cx_ptr: *const TaskContext
# ) -> usize
    .section .text
    .globl __switch
__switch:
    # save ra and sp
    sd ra, 0(a0)
    sd sp, 8(a0)
    # save callee-saved registers
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    
    # load ra and sp
    ld ra, 0(a1)
    ld sp, 8(a1)
    # load callee-saved registers
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr

    mv a0, sp
    ret

