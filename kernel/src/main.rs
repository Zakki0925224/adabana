#![no_std]
#![no_main]

mod addr;
mod asm;
mod boot;
mod gpio;
mod panic;
mod uart;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let uart = uart::MiniUart::new();
    uart.init();
    uart.puts("Hello, world!\n");

    loop {
        uart.send(uart.receive());
    }
}
