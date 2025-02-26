use crate::addr::MmioAddress;
use core::arch::asm;

// TODO: volatile read/write was not working
pub fn read_mmio(addr: &MmioAddress) -> u32 {
    let value: u32;
    let ptr = addr.get() as *const u32;
    unsafe {
        // value = ptr.read_volatile();
        value = *ptr;
    }
    value
}

pub fn write_mmio(addr: &MmioAddress, value: u32) {
    let ptr = addr.get() as *mut u32;
    unsafe {
        // ptr.write_volatile(value);
        *ptr = value;
    }
}

pub fn wait_cycles(cycles: usize) {
    for _ in 0..cycles {
        unsafe { asm!("nop") }
    }
}
