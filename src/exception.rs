use core::arch::{asm, global_asm};

use crate::uart;

const EXCEPTION_FRAME_SIZE: usize = core::mem::size_of::<ExceptionFrame>();

global_asm!(
    r#"
    .section .boot.text.exceptions, "ax"
    .align 11
    .global __exception_vectors
__exception_vectors:
    b __vector_current_el_sp0_sync
    .balign 128
    b __vector_current_el_sp0_irq
    .balign 128
    b __vector_current_el_sp0_fiq
    .balign 128
    b __vector_current_el_sp0_serror
    .balign 128
    b __vector_current_el_spx_sync
    .balign 128
    b __vector_current_el_spx_irq
    .balign 128
    b __vector_current_el_spx_fiq
    .balign 128
    b __vector_current_el_spx_serror
    .balign 128
    b __vector_lower_el_a64_sync
    .balign 128
    b __vector_lower_el_a64_irq
    .balign 128
    b __vector_lower_el_a64_fiq
    .balign 128
    b __vector_lower_el_a64_serror
    .balign 128
    b __vector_lower_el_a32_sync
    .balign 128
    b __vector_lower_el_a32_irq
    .balign 128
    b __vector_lower_el_a32_fiq
    .balign 128
    b __vector_lower_el_a32_serror

    .macro save_and_return vector_id
    sub sp, sp, #288
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    mov x9, #\vector_id
    str x9, [sp, #248]
    mrs x9, esr_el1
    str x9, [sp, #256]
    mrs x9, far_el1
    str x9, [sp, #264]
    mrs x9, elr_el1
    str x9, [sp, #272]
    mrs x9, spsr_el1
    str x9, [sp, #280]

    mov x0, sp
    bl rust_exception_handler

    ldr x9, [sp, #272]
    msr elr_el1, x9
    ldr x9, [sp, #280]
    msr spsr_el1, x9

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]
    add sp, sp, #288
    eret
    .endm

__vector_current_el_sp0_sync:
    save_and_return 0
__vector_current_el_sp0_irq:
    save_and_return 1
__vector_current_el_sp0_fiq:
    save_and_return 2
__vector_current_el_sp0_serror:
    save_and_return 3
__vector_current_el_spx_sync:
    save_and_return 4
__vector_current_el_spx_irq:
    save_and_return 5
__vector_current_el_spx_fiq:
    save_and_return 6
__vector_current_el_spx_serror:
    save_and_return 7
__vector_lower_el_a64_sync:
    save_and_return 8
__vector_lower_el_a64_irq:
    save_and_return 9
__vector_lower_el_a64_fiq:
    save_and_return 10
__vector_lower_el_a64_serror:
    save_and_return 11
__vector_lower_el_a32_sync:
    save_and_return 12
__vector_lower_el_a32_irq:
    save_and_return 13
__vector_lower_el_a32_fiq:
    save_and_return 14
__vector_lower_el_a32_serror:
    save_and_return 15

    .align 11
    .global __exception_vectors_el2
__exception_vectors_el2:
    b __vector_el2_current_el_sp0_sync
    .balign 128
    b __vector_el2_current_el_sp0_irq
    .balign 128
    b __vector_el2_current_el_sp0_fiq
    .balign 128
    b __vector_el2_current_el_sp0_serror
    .balign 128
    b __vector_el2_current_el_spx_sync
    .balign 128
    b __vector_el2_current_el_spx_irq
    .balign 128
    b __vector_el2_current_el_spx_fiq
    .balign 128
    b __vector_el2_current_el_spx_serror
    .balign 128
    b __vector_el2_lower_el_a64_sync
    .balign 128
    b __vector_el2_lower_el_a64_irq
    .balign 128
    b __vector_el2_lower_el_a64_fiq
    .balign 128
    b __vector_el2_lower_el_a64_serror
    .balign 128
    b __vector_el2_lower_el_a32_sync
    .balign 128
    b __vector_el2_lower_el_a32_irq
    .balign 128
    b __vector_el2_lower_el_a32_fiq
    .balign 128
    b __vector_el2_lower_el_a32_serror

    .macro save_and_return_el2 vector_id
    sub sp, sp, #288
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    mov x9, #\vector_id
    str x9, [sp, #248]
    mrs x9, esr_el2
    str x9, [sp, #256]
    mrs x9, far_el2
    str x9, [sp, #264]
    mrs x9, elr_el2
    str x9, [sp, #272]
    mrs x9, spsr_el2
    str x9, [sp, #280]

    mov x0, sp
    bl rust_exception_handler

    ldr x9, [sp, #272]
    msr elr_el2, x9
    ldr x9, [sp, #280]
    msr spsr_el2, x9

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]
    add sp, sp, #288
    eret
    .endm

__vector_el2_current_el_sp0_sync:
    save_and_return_el2 0
__vector_el2_current_el_sp0_irq:
    save_and_return_el2 1
__vector_el2_current_el_sp0_fiq:
    save_and_return_el2 2
__vector_el2_current_el_sp0_serror:
    save_and_return_el2 3
__vector_el2_current_el_spx_sync:
    save_and_return_el2 4
__vector_el2_current_el_spx_irq:
    save_and_return_el2 5
__vector_el2_current_el_spx_fiq:
    save_and_return_el2 6
__vector_el2_current_el_spx_serror:
    save_and_return_el2 7
__vector_el2_lower_el_a64_sync:
    save_and_return_el2 8
__vector_el2_lower_el_a64_irq:
    save_and_return_el2 9
__vector_el2_lower_el_a64_fiq:
    save_and_return_el2 10
__vector_el2_lower_el_a64_serror:
    save_and_return_el2 11
__vector_el2_lower_el_a32_sync:
    save_and_return_el2 12
__vector_el2_lower_el_a32_irq:
    save_and_return_el2 13
__vector_el2_lower_el_a32_fiq:
    save_and_return_el2 14
__vector_el2_lower_el_a32_serror:
    save_and_return_el2 15
"#
);

unsafe extern "C" {
    static __exception_vectors: u8;
    static __exception_vectors_el2: u8;
}

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn exception_init_el2() {
    unsafe {
        let vbar = core::ptr::addr_of!(__exception_vectors_el2) as u64;
        asm!(
            "msr vbar_el2, {vbar}",
            "isb",
            vbar = in(reg) vbar,
            options(nostack, preserves_flags)
        );
    }
}

#[link_section = ".boot.text"]
pub fn init() {
    unsafe {
        let vbar = core::ptr::addr_of!(__exception_vectors) as u64;
        asm!(
            "msr vbar_el1, {vbar}",
            "isb",
            vbar = in(reg) vbar,
            options(nostack, preserves_flags)
        );
    }
}

#[repr(C)]
pub struct ExceptionFrame {
    pub regs: [u64; 31],
    pub vector: u64,
    pub esr: u64,
    pub far: u64,
    pub elr: u64,
    pub spsr: u64,
}

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn rust_exception_handler(frame: &mut ExceptionFrame) {
    uart::early_puts(b"\r\nexception: ");
    put_vector_name(frame.vector);
    uart::early_puts(b"\r\n");

    uart::early_puts(b"esr=");
    uart::early_put_hex_u64(frame.esr);
    uart::early_puts(b"\r\nfar=");
    uart::early_put_hex_u64(frame.far);
    uart::early_puts(b"\r\nelr=");
    uart::early_put_hex_u64(frame.elr);
    uart::early_puts(b"\r\nspsr=");
    uart::early_put_hex_u64(frame.spsr);
    uart::early_puts(b"\r\n");

    uart::early_puts(b"decoded: ");
    uart::early_puts(fault_name(frame.esr));
    uart::early_puts(b"\r\n");

    if is_address_size_fault(frame.esr) {
        uart::early_puts(b"Address Size Fault\r\n");
    }

    if is_translation_fault(frame.esr) {
        uart::early_puts(b"Translation Fault\r\n");
    }

    #[cfg(any(translation_fault_test, dram_oob_test))]
    if should_resume_after_test(frame) {
        uart::early_puts(b"test enabled: resume at next instruction\r\n");
        frame.elr = frame.elr.wrapping_add(4);
        return;
    }

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(any(translation_fault_test, dram_oob_test))]
#[link_section = ".boot.text"]
fn should_resume_after_test(frame: &ExceptionFrame) -> bool {
    is_synchronous_vector(frame.vector) && is_abort_exception(frame.esr)
}

#[cfg(any(translation_fault_test, dram_oob_test))]
#[link_section = ".boot.text"]
fn is_synchronous_vector(vector: u64) -> bool {
    matches!(vector, 0 | 4 | 8 | 12)
}

#[link_section = ".boot.text"]
fn is_abort_exception(esr: u64) -> bool {
    matches!((esr >> 26) & 0x3f, 0x20 | 0x21 | 0x24 | 0x25)
}

#[link_section = ".boot.text"]
fn put_vector_name(vector: u64) {
    match vector {
        0 => uart::early_puts(b"current_el_sp0_sync"),
        1 => uart::early_puts(b"current_el_sp0_irq"),
        2 => uart::early_puts(b"current_el_sp0_fiq"),
        3 => uart::early_puts(b"current_el_sp0_serror"),
        4 => uart::early_puts(b"current_el_spx_sync"),
        5 => uart::early_puts(b"current_el_spx_irq"),
        6 => uart::early_puts(b"current_el_spx_fiq"),
        7 => uart::early_puts(b"current_el_spx_serror"),
        8 => uart::early_puts(b"lower_el_a64_sync"),
        9 => uart::early_puts(b"lower_el_a64_irq"),
        10 => uart::early_puts(b"lower_el_a64_fiq"),
        11 => uart::early_puts(b"lower_el_a64_serror"),
        12 => uart::early_puts(b"lower_el_a32_sync"),
        13 => uart::early_puts(b"lower_el_a32_irq"),
        14 => uart::early_puts(b"lower_el_a32_fiq"),
        15 => uart::early_puts(b"lower_el_a32_serror"),
        _ => uart::early_puts(b"unknown"),
    }
}

#[link_section = ".boot.text"]
fn is_translation_fault(esr: u64) -> bool {
    let ec = (esr >> 26) & 0x3f;
    let fsc = esr & 0x3f;

    matches!(ec, 0x20 | 0x21 | 0x24 | 0x25) && matches!(fsc, 0b000100..=0b000111)
}

#[link_section = ".boot.text"]
fn is_address_size_fault(esr: u64) -> bool {
    let ec = (esr >> 26) & 0x3f;
    let fsc = esr & 0x3f;

    matches!(ec, 0x20 | 0x21 | 0x24 | 0x25) && matches!(fsc, 0b000000..=0b000011)
}

#[link_section = ".boot.text"]
fn fault_name(esr: u64) -> &'static [u8] {
    let ec = (esr >> 26) & 0x3f;
    let fsc = esr & 0x3f;

    match ec {
        0x20 | 0x21 | 0x24 | 0x25 => match fsc {
            0b000000..=0b000011 => b"address size fault",
            0b000100..=0b000111 => b"translation fault",
            0b001000..=0b001011 => b"access flag fault",
            0b001100..=0b001111 => b"permission fault",
            0b010000 => b"synchronous external abort",
            0b010100..=0b010111 => b"synchronous external abort on table walk",
            0b011000 => b"parity or ecc error",
            0b011100..=0b011111 => b"parity or ecc error on table walk",
            0b100001 => b"alignment fault",
            0b110000 => b"tlb conflict abort",
            _ => b"other abort",
        },
        _ => b"unknown",
    }
}

const _: () = assert!(EXCEPTION_FRAME_SIZE == 288);