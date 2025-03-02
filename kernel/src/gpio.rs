use crate::addr::MmioAddress;

fn mmio_base_gpio() -> MmioAddress {
    MmioAddress::new(0x200000)
}

pub fn read_gpfsel1() -> u32 {
    mmio_base_gpio().offset(0x04).read()
}

pub fn write_gpfsel1(value: u32) {
    mmio_base_gpio().offset(0x04).write(value);
}

pub fn read_gppud() -> u32 {
    mmio_base_gpio().offset(0x94).read()
}

pub fn write_gppud(value: u32) {
    mmio_base_gpio().offset(0x94).write(value);
}

pub fn read_gppudclk0() -> u32 {
    mmio_base_gpio().offset(0x98).read()
}

pub fn write_gppudclk0(value: u32) {
    mmio_base_gpio().offset(0x98).write(value);
}
