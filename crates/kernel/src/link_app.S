
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 17
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_9_start
    .quad app_10_start
    .quad app_11_start
    .quad app_12_start
    .quad app_13_start
    .quad app_14_start
    .quad app_15_start
    .quad app_16_start
    .quad app_16_end

    .global _app_names
_app_names:
    .string "exit"
    .string "fantastic_text"
    .string "forkexec"
    .string "forktest"
    .string "forktest2"
    .string "forktest_simple"
    .string "forktree"
    .string "hello_world"
    .string "initproc"
    .string "matrix"
    .string "sleep"
    .string "sleep_simple"
    .string "stack_overflow"
    .string "user_shell"
    .string "usertests"
    .string "usertests-simple"
    .string "yield"

    .section .data
    .global app_0_start
    .global app_0_end
    .align 3
app_0_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/exit"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 3
app_1_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/fantastic_text"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 3
app_2_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/forkexec"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .align 3
app_3_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/forktest"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
    .align 3
app_4_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/forktest2"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
    .align 3
app_5_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/forktest_simple"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
    .align 3
app_6_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/forktree"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
    .align 3
app_7_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/hello_world"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
    .align 3
app_8_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/initproc"
app_8_end:

    .section .data
    .global app_9_start
    .global app_9_end
    .align 3
app_9_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/matrix"
app_9_end:

    .section .data
    .global app_10_start
    .global app_10_end
    .align 3
app_10_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/sleep"
app_10_end:

    .section .data
    .global app_11_start
    .global app_11_end
    .align 3
app_11_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/sleep_simple"
app_11_end:

    .section .data
    .global app_12_start
    .global app_12_end
    .align 3
app_12_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/stack_overflow"
app_12_end:

    .section .data
    .global app_13_start
    .global app_13_end
    .align 3
app_13_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/user_shell"
app_13_end:

    .section .data
    .global app_14_start
    .global app_14_end
    .align 3
app_14_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/usertests"
app_14_end:

    .section .data
    .global app_15_start
    .global app_15_end
    .align 3
app_15_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/usertests-simple"
app_15_end:

    .section .data
    .global app_16_start
    .global app_16_end
    .align 3
app_16_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/yield"
app_16_end:

    .align 3
    .section .data
    .global _num_service
_num_service:
    .quad 1
    .quad service_0_start
    .quad service_0_end

    .global _service_names
_service_names:
    .string "pm"

    .section .data
    .global service_0_start
    .global service_0_end
    .align 3
service_0_start:
    .incbin "/home/conless/Desktop/study/os/sjtu-cs2952/remire/target/riscv64gc-unknown-none-elf/debug/pm"
service_0_end:
