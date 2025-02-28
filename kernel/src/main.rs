#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

mod addr;
mod asm;
mod boot;
mod console;
mod error;
mod gpio;
mod mutex;
mod panic;
mod uart;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    kernel_main2().unwrap();
    loop {}
}

fn kernel_main2() -> error::Result<()> {
    uart::init()?;

    println!("Hello, world!");
    loop {
        let c = uart::receive()?;
        uart::send(c)?;
    }
}
