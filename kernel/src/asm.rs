use crate::addr::MmioAddress;
use core::arch::asm;

// TODO: volatile read/write was not working
pub fn read_mmio(addr: &MmioAddress) -> u32 {
    let value;
    let ptr = addr.get() as *const u32;
    unsafe {
        value = ptr.read_volatile();
    }
    value
}

pub fn write_mmio(addr: &MmioAddress, value: u32) {
    let ptr = addr.get() as *mut u32;
    unsafe {
        ptr.write_volatile(value);
    }
}

pub fn wait_cycles(cycles: usize) {
    for _ in 0..cycles {
        unsafe { asm!("nop") }
    }
}

pub fn read_main_id_reg() -> u32 {
    let value;

    unsafe {
        asm!("mrs {0:x}, midr_el1", out(reg) value);
    }

    value
}

pub fn read_x0() -> u64 {
    let value;

    unsafe {
        asm!("mov {0}, x0", out(reg) value);
    }

    value
}

pub fn disabled_int<F: FnMut() -> R, R>(mut func: F) -> R {
    unsafe { asm!("msr daifset, #3") }; // disable IRQ and FIQ interrupts
    let res = func();
    unsafe { asm!("msr daifclr, #3") }; // enable IRQ and FIQ interrupts
    res
}
