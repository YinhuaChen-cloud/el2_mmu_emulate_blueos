use core::arch::global_asm;

global_asm!(
    r#"
    .section .boot.text, "ax"
    .global _start
    .global _el1_start
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
    mrs x2, CurrentEL
    cmp x2, #0x8
    b.ne _el1_start

    bl exception_init_el2
    bl enable_el2_mmu
    // 经过 gdb 调试，发现访问 0x8000_0000 会触发 EL2 的 translation fault 异常，说明 EL2 MMU 已经成功启用
    // bl el2_translation_fault_test // (此时运行测试会在 rust_exception_handler 卡死，因为部分字符串在高 VA)

    // set HCR_EL2 to enable AArch64 execution in EL1
    mov x3, #(1 << 31)
    msr hcr_el2, x3
    mov x3, #0x3c5
    msr spsr_el2, x3
    msr sp_el1, x0
    ldr x3, =_el1_start
    msr elr_el2, x3
    isb
    eret

_el1_start:
    bl init
2:
    wfe
    b 2b
"#
);


