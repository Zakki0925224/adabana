use crate::addr::MmioAddress;

const MMIO_BASE_GPIO: MmioAddress = MmioAddress::new(0x200000);

pub fn read_gpfsel1() -> u32 {
    MMIO_BASE_GPIO.offset(0x04).read()
}

pub fn write_gpfsel1(value: u32) {
    MMIO_BASE_GPIO.offset(0x04).write(value);
}

pub fn read_gppud() -> u32 {
    MMIO_BASE_GPIO.offset(0x94).read()
}

pub fn write_gppud(value: u32) {
    MMIO_BASE_GPIO.offset(0x94).write(value);
}

pub fn read_gppudclk0() -> u32 {
    MMIO_BASE_GPIO.offset(0x98).read()
}

pub fn write_gppudclk0(value: u32) {
    MMIO_BASE_GPIO.offset(0x98).write(value);
}
