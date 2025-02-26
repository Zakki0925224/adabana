use crate::asm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MmioAddress(u32);

impl MmioAddress {
    pub const MMIO_BASE_RPI3: u32 = 0x3f000000;

    pub const fn new(offset: u32) -> Self {
        Self(Self::MMIO_BASE_RPI3 + offset)
    }

    pub fn get(self) -> u32 {
        self.0
    }

    pub fn offset(self, offset: usize) -> Self {
        Self(self.0 + offset as u32)
    }

    pub fn read(&self) -> u32 {
        asm::read_mmio(self)
    }

    pub fn write(&self, value: u32) {
        asm::write_mmio(self, value);
    }
}
