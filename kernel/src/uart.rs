use crate::{addr::MmioAddress, asm, error::Result, gpio, mutex::Mutex};

const MMIO_BASE_UART1: MmioAddress = MmioAddress::new(0x215000);

static mut MINI_UART: Mutex<MiniUart> = Mutex::new(MiniUart::new());

struct IoRegister(MmioAddress);

impl IoRegister {
    const fn new(base: MmioAddress) -> Self {
        Self(base)
    }

    fn io_port_base(&self) -> &MmioAddress {
        &self.0
    }

    fn write_aux_enables(&self, value: u32) {
        self.io_port_base().offset(0x04).write(value);
    }

    fn read_aux_mu_io(&self) -> u32 {
        self.io_port_base().offset(0x40).read()
    }

    fn write_aux_mu_io(&self, value: u32) {
        self.io_port_base().offset(0x40).write(value);
    }

    // interrupt enable
    fn write_aux_mu_ier(&self, value: u32) {
        self.io_port_base().offset(0x44).write(value);
    }

    // interrupt identify
    fn write_aux_mu_iir(&self, value: u32) {
        self.io_port_base().offset(0x48).write(value);
    }

    // line control
    fn write_aux_mu_lcr(&self, value: u32) {
        self.io_port_base().offset(0x4c).write(value);
    }

    // modem control
    fn write_aux_mu_mcr(&self, value: u32) {
        self.io_port_base().offset(0x50).write(value);
    }

    // line status
    fn read_aux_mu_lsr(&self) -> u32 {
        self.io_port_base().offset(0x54).read()
    }

    // extra control
    fn write_aux_mu_cntl(&self, value: u32) {
        self.io_port_base().offset(0x60).write(value);
    }

    // baudrate
    fn write_aux_mu_baud(&self, value: u32) {
        self.io_port_base().offset(0x68).write(value);
    }
}

struct MiniUart {
    io_register: IoRegister,
}

impl MiniUart {
    pub const fn new() -> Self {
        Self {
            io_register: IoRegister::new(MMIO_BASE_UART1),
        }
    }

    fn io_register(&self) -> &IoRegister {
        &self.io_register
    }

    fn init(&self) {
        let io_register = self.io_register();

        io_register.write_aux_enables(0x01);
        io_register.write_aux_mu_ier(0x00);
        io_register.write_aux_mu_cntl(0x00);
        io_register.write_aux_mu_lcr(0x03); // 8-bit mode
        io_register.write_aux_mu_mcr(0x00);
        io_register.write_aux_mu_ier(0x00);
        io_register.write_aux_mu_iir(0x03);
        io_register.write_aux_mu_baud(270); // 115200 baudrate

        // map to GPIO pins
        let mut gpfsel1 = gpio::read_gpfsel1();
        gpfsel1 |= 0b010 << 12; // set alt5 for GPIO14
        gpfsel1 |= 0b010 << 15; // set alt5 for GPIO15
        gpio::write_gpfsel1(gpfsel1);

        gpio::write_gppud(0);
        asm::wait_cycles(150);

        gpio::write_gppudclk0(1 << 14 | 1 << 15); // assert clock
        asm::wait_cycles(150);
        gpio::write_gppudclk0(0); // deassert clock

        io_register.write_aux_mu_cntl(0x03); // enable TX and RX
    }

    fn send(&self, c: char) {
        if c == '\n' {
            self.send('\r');
        }

        let io_register = self.io_register();

        // wait
        loop {
            if io_register.read_aux_mu_lsr() & 0x20 != 0 {
                break;
            }

            asm::wait_cycles(1);
        }

        io_register.write_aux_mu_io(c as u32);
    }

    fn receive(&self) -> char {
        let io_register = self.io_register();

        // wait
        loop {
            if io_register.read_aux_mu_lsr() & 0x01 != 0 {
                break;
            }

            asm::wait_cycles(1);
        }

        let mut c = io_register.read_aux_mu_io() as u8 as char;

        if c == '\r' {
            c = '\n';
        }

        c
    }

    fn puts(&self, s: &str) {
        for c in s.chars() {
            self.send(c);
        }
    }
}

pub fn init() -> Result<()> {
    unsafe { MINI_UART.try_lock() }?.init();
    Ok(())
}

pub fn receive() -> Result<char> {
    let c = unsafe { MINI_UART.try_lock() }?.receive();
    Ok(c)
}

pub fn send(c: char) -> Result<()> {
    unsafe { MINI_UART.try_lock() }?.send(c);
    Ok(())
}

pub fn puts(s: &str) -> Result<()> {
    unsafe { MINI_UART.try_lock() }?.puts(s);
    Ok(())
}
