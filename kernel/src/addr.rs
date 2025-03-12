use crate::{asm, cpu};
use core::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MmioAddress(u32);

impl MmioAddress {
    pub fn new(offset: u32) -> Self {
        Self(cpu::mmio_base() + offset)
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

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

impl From<u64> for VirtualAddress {
    fn from(addr: u64) -> Self {
        Self::new(addr)
    }
}

impl Display for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl Debug for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

impl VirtualAddress {
    pub fn new(addr: u64) -> Self {
        Self(addr)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub fn offset(self, offset: usize) -> Self {
        Self(self.get() + offset as u64)
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    pub fn as_ptr_mut<T>(&self) -> *mut T {
        self.0 as *mut T
    }
}
