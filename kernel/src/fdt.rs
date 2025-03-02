#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

impl FdtHeader {
    pub fn magic(&self) -> u32 {
        self.magic.to_be()
    }

    pub fn totalsize(&self) -> u32 {
        self.totalsize.to_be()
    }

    pub fn is_valid(&self) -> bool {
        self.magic() == 0xd00dfeed
    }
}
