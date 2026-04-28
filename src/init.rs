#![no_std]
#![no_main]
#![allow(special_module_name)]

use core::panic::PanicInfo;

mod exception;
mod main;
mod mmu;
mod start;
mod uart;

#[link_section = ".boot.rodata"]
static INIT_MSG: [u8; b"we are just in init()\r\n".len()] = *b"we are just in init()\r\n";
#[link_section = ".boot.rodata"]
static MMU_ENABLED_MSG: [u8; b"mmu enabled\r\n".len()] = *b"mmu enabled\r\n";
#[cfg(translation_fault_test)]
#[link_section = ".boot.rodata"]
static EL2_TRANSLATION_FAULT_TEST_ENABLED_MSG: [u8; b"el2 translation fault test enabled\r\n".len()] =
    *b"el2 translation fault test enabled\r\n";
#[cfg(translation_fault_test)]
#[link_section = ".boot.rodata"]
static EL2_TRANSLATION_FAULT_READ_MSG: [u8; b"el2 read 0x8000_0000 -> expect translation fault\r\n".len()] =
    *b"el2 read 0x8000_0000 -> expect translation fault\r\n";
#[cfg(translation_fault_test)]
#[link_section = ".boot.rodata"]
static EL2_TRANSLATION_FAULT_RETURNED_MSG: [u8; b"returned from el2 translation-fault handler\r\n".len()] =
    *b"returned from el2 translation-fault handler\r\n";
#[cfg(dram_oob_test)]
#[link_section = ".boot.rodata"]
static DRAM_OOB_TEST_ENABLED_MSG: [u8; b"dram out-of-range test enabled\r\n".len()] =
    *b"dram out-of-range test enabled\r\n";
#[cfg(dram_oob_test)]
#[link_section = ".boot.rodata"]
static DRAM_OOB_READ_MSG: [u8; b"read 0xa000_0000 -> verify actual fault type\r\n".len()] =
    *b"read 0xa000_0000 -> verify actual fault type\r\n";
#[cfg(dram_oob_test)]
#[link_section = ".boot.rodata"]
static DRAM_OOB_RETURNED_MSG: [u8; b"returned from out-of-range handler\r\n".len()] =
    *b"returned from out-of-range handler\r\n";
#[cfg(not(dram_oob_test))]
#[link_section = ".boot.rodata"]
static EL1_EXCEPTION_TEST_DISABLED_MSG: [u8; b"no el1 exception test enabled\r\n".len()] =
    *b"no el1 exception test enabled\r\n";

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn el2_translation_fault_test() {
    #[cfg(translation_fault_test)]
    {
        uart::early_puts(&EL2_TRANSLATION_FAULT_TEST_ENABLED_MSG);
        uart::early_puts(&EL2_TRANSLATION_FAULT_READ_MSG);

        unsafe {
            core::ptr::read_volatile(0x8000_0000 as *const u64);
        }

        uart::early_puts(&EL2_TRANSLATION_FAULT_RETURNED_MSG);
    }
}

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn init() -> ! {
    uart::early_puts(&INIT_MSG);
    exception::init();
    mmu::init();
    uart::early_puts(&MMU_ENABLED_MSG);

    #[cfg(dram_oob_test)]
    {
        uart::early_puts(&DRAM_OOB_TEST_ENABLED_MSG);
        uart::early_puts(&DRAM_OOB_READ_MSG);

        unsafe {
            core::ptr::read_volatile(0xa000_0000 as *const u64);
        }

        uart::early_puts(&DRAM_OOB_RETURNED_MSG);
    }

    #[cfg(not(dram_oob_test))]
    uart::early_puts(&EL1_EXCEPTION_TEST_DISABLED_MSG);

    jump_to_high_kernel(main::main)
}

#[link_section = ".boot.text"]
fn jump_to_high_kernel(entry: extern "C" fn() -> !) -> ! {
    unsafe {
        core::arch::asm!(
            "br {entry}",
            entry = in(reg) entry,
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}