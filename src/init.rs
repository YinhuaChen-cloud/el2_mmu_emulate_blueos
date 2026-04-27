#![no_std]
#![no_main]
#![allow(special_module_name)]

use core::panic::PanicInfo;

mod exception;
mod main;
mod mmu;
mod start;
mod uart;

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn init() -> ! {
    uart::early_puts(b"we are just in init()\r\n");
    exception::init();
    mmu::init();
    uart::early_puts(b"mmu enabled\r\n");

    #[cfg(translation_fault_test)]
    {
        uart::early_puts(b"translation fault test enabled\r\n");
        uart::early_puts(b"read 0x8000_0000 -> expect translation fault\r\n");

        unsafe {
            core::ptr::read_volatile(0x8000_0000 as *const u64);
        }

        uart::early_puts(b"returned from translation-fault handler\r\n");
    }

    #[cfg(dram_oob_test)]
    {
        uart::early_puts(b"dram out-of-range test enabled\r\n");
        uart::early_puts(b"read 0xa000_0000 -> verify actual fault type\r\n");

        unsafe {
            core::ptr::read_volatile(0xa000_0000 as *const u64);
        }

        uart::early_puts(b"returned from out-of-range handler\r\n");
    }

    #[cfg(not(any(translation_fault_test, dram_oob_test)))]
    uart::early_puts(b"exception test disabled\r\n");

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