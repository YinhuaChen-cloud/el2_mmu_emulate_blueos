use core::arch::global_asm;

global_asm!(
    r#"
    .section .boot.text, "ax"
    .global _start
_start:
    mrs x1, mpidr_el1
    and x1, x1, #0xff
    cbz x1, 0f
1:
    wfe
    b 1b
0:
    ldr x0, =__boot_stack_top
    mov sp, x0
    bl init
2:
    wfe
    b 2b
"#
);
