use core::arch::asm;

const ENTRY_COUNT: usize = 512;
const PAGE_SHIFT_2M: usize = 21;
const PAGE_SHIFT_1G: usize = 30;
pub const LINEAR_MAP_BASE: usize = 0xffff_ffff_0000_0000;
const LINEAR_L1_START_INDEX: usize = 0;
const LINEAR_MAP_GB_COUNT: usize = 4;
const OOB_TEST_VA: usize = 0xa000_0000;

const DESC_VALID: u64 = 1 << 0;
const DESC_TABLE: u64 = 1 << 1;
const DESC_BLOCK: u64 = 0 << 1;

const ATTR_IDX_SHIFT: u64 = 2;
const ATTR_IDX_DEVICE: u64 = 0 << ATTR_IDX_SHIFT;
const ATTR_IDX_NORMAL: u64 = 1 << ATTR_IDX_SHIFT;
const ACCESS_FLAG: u64 = 1 << 10;
const INNER_SHAREABLE: u64 = 0b11 << 8;
const PXN: u64 = 1 << 53;
const UXN: u64 = 1 << 54;

const MAIR_DEVICE_NGNRNE: u64 = 0x00;
const MAIR_NORMAL_WB: u64 = 0xff;
const MAIR_VALUE: u64 = MAIR_DEVICE_NGNRNE | (MAIR_NORMAL_WB << 8);

const TCR_T0SZ_4GB: u64 = 32;
const TCR_T1SZ_4GB: u64 = 32 << 16;
const TCR_IRGN0_WBWA: u64 = 0b01 << 8;
const TCR_ORGN0_WBWA: u64 = 0b01 << 10;
const TCR_SH0_INNER: u64 = 0b11 << 12;
const TCR_TG0_4K: u64 = 0b00 << 14;
const TCR_EL2_PS_40BIT: u64 = 0b010 << 16;
const TCR_IRGN1_WBWA: u64 = 0b01 << 24;
const TCR_ORGN1_WBWA: u64 = 0b01 << 26;
const TCR_SH1_INNER: u64 = 0b11 << 28;
const TCR_TG1_4K: u64 = 0b10 << 30;
const TCR_IPS_40BIT: u64 = 0b010 << 32;
const TCR_EL2_VALUE: u64 =
    TCR_T0SZ_4GB | TCR_IRGN0_WBWA | TCR_ORGN0_WBWA | TCR_SH0_INNER | TCR_TG0_4K | TCR_EL2_PS_40BIT;
const TCR_VALUE: u64 = TCR_T0SZ_4GB
    | TCR_T1SZ_4GB
    | TCR_IRGN0_WBWA
    | TCR_ORGN0_WBWA
    | TCR_SH0_INNER
    | TCR_TG0_4K
    | TCR_IRGN1_WBWA
    | TCR_ORGN1_WBWA
    | TCR_SH1_INNER
    | TCR_TG1_4K
    | TCR_IPS_40BIT;

#[repr(C, align(4096))]
struct PageTable([u64; ENTRY_COUNT]);

#[link_section = ".boot.bss.pgtables"]
static mut L1_TABLE: PageTable = PageTable([0; ENTRY_COUNT]);
#[link_section = ".boot.bss.pgtables"]
static mut LINEAR_L1_TABLE: PageTable = PageTable([0; ENTRY_COUNT]);
#[link_section = ".boot.bss.pgtables"]
static mut LOW_1GB_L2_TABLE: PageTable = PageTable([0; ENTRY_COUNT]);
#[cfg(dram_oob_test)]
#[link_section = ".boot.bss.pgtables"]
static mut OOB_1GB_L2_TABLE: PageTable = PageTable([0; ENTRY_COUNT]);

#[inline(always)]
const fn table_desc(addr: usize) -> u64 {
    (addr as u64) | DESC_VALID | DESC_TABLE
}

#[inline(always)]
const fn block_desc(addr: usize, attrs: u64) -> u64 {
    (addr as u64) | DESC_VALID | DESC_BLOCK | attrs
}

#[inline(always)]
const fn device_block_attrs() -> u64 {
    ATTR_IDX_DEVICE | ACCESS_FLAG | PXN | UXN
}

#[inline(always)]
const fn normal_block_attrs() -> u64 {
    ATTR_IDX_NORMAL | ACCESS_FLAG | INNER_SHAREABLE
}

#[link_section = ".boot.text"]
unsafe fn build_identity_map() {
    L1_TABLE.0[0] = table_desc(core::ptr::addr_of!(LOW_1GB_L2_TABLE) as usize);

    for index in 0..ENTRY_COUNT {
        let phys = index << PAGE_SHIFT_2M;
        LOW_1GB_L2_TABLE.0[index] = block_desc(phys, device_block_attrs());
    }

    // 左闭右开区间 等价于 >= 1 && < 2
    for index in 1..2 {
        let phys = index << PAGE_SHIFT_1G;
        L1_TABLE.0[index] = block_desc(phys, normal_block_attrs());
    }

    #[cfg(dram_oob_test)]
    {
        let l1_index = OOB_TEST_VA >> PAGE_SHIFT_1G;
        let l2_index = (OOB_TEST_VA & ((1usize << PAGE_SHIFT_1G) - 1)) >> PAGE_SHIFT_2M;

        L1_TABLE.0[l1_index] = table_desc(core::ptr::addr_of!(OOB_1GB_L2_TABLE) as usize);
        OOB_1GB_L2_TABLE.0[l2_index] = block_desc(OOB_TEST_VA, normal_block_attrs());
    }
}

#[link_section = ".boot.text"]
unsafe fn build_linear_map() {
    // TTBR1 uses the top 4GB VA region selected by T1SZ=32.
    // Within that region, the walk starts from L1 and uses VA[31:30],
    // so the 4 x 1GB blocks must live in L1 entries 0..3.
    for index in 0..LINEAR_MAP_GB_COUNT {
        let phys = index << PAGE_SHIFT_1G;
        let attrs = if index == 0 {
            device_block_attrs()
        } else {
            normal_block_attrs()
        };

        LINEAR_L1_TABLE.0[LINEAR_L1_START_INDEX + index] = block_desc(phys, attrs);
    }
}

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn enable_el2_mmu() {
    unsafe {
        build_identity_map();

        let ttbr0 = core::ptr::addr_of!(L1_TABLE) as u64;
        let mut sctlr: u64;

        asm!(
            "dsb ishst",
            "msr mair_el2, {mair}",
            "msr tcr_el2, {tcr}",
            "msr ttbr0_el2, {ttbr0}",
            "isb",
            "tlbi alle2",
            "dsb ish",
            "isb",
            "mrs {sctlr}, sctlr_el2",
            mair = in(reg) MAIR_VALUE,
            tcr = in(reg) TCR_EL2_VALUE,
            ttbr0 = in(reg) ttbr0,
            sctlr = out(reg) sctlr,
            options(nostack)
        );

        sctlr |= 1 << 0;

        asm!(
            "msr sctlr_el2, {sctlr}",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack)
        );
    }
}

#[link_section = ".boot.text"]
pub fn init() {
    unsafe {
        build_identity_map();
        build_linear_map();

        let ttbr0 = core::ptr::addr_of!(L1_TABLE) as u64;
        let ttbr1 = core::ptr::addr_of!(LINEAR_L1_TABLE) as u64;
        let mut sctlr: u64;

        asm!(
            "dsb ishst",
            "msr mair_el1, {mair}",
            "msr tcr_el1, {tcr}",
            "msr ttbr0_el1, {ttbr0}",
            "msr ttbr1_el1, {ttbr1}",
            "isb",
            "tlbi vmalle1",
            "dsb ish",
            "isb",
            "mrs {sctlr}, sctlr_el1",
            mair = in(reg) MAIR_VALUE,
            tcr = in(reg) TCR_VALUE,
            ttbr0 = in(reg) ttbr0,
            ttbr1 = in(reg) ttbr1,
            sctlr = out(reg) sctlr,
            options(nostack)
        );

        sctlr |= 1 << 0;

        asm!(
            "msr sctlr_el1, {sctlr}",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack)
        );
    }
}