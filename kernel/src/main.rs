#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use addr::VirtualAddress;
use device_tree::DeviceTree;

mod addr;
mod asm;
mod boot;
mod console;
mod cpu;
mod device_tree;
mod error;
mod fdt;
mod gpio;
mod mailbox;
mod mutex;
mod panic;
mod uart;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let fdt_addr = asm::read_x0();
    assert!(fdt_addr != 0);
    kernel_main2(fdt_addr.into()).unwrap();
    unreachable!();
}

fn kernel_main2(fdt_addr: VirtualAddress) -> error::Result<()> {
    let _device_tree = DeviceTree::new(fdt_addr)?;
    let cpu_model = cpu::detect_cpu_model()?;

    uart::init()?;

    println!("Starting kernel...");
    println!("CPU: {:?}", cpu_model);
    println!(
        "Firmware revision: 0x{:x}",
        mailbox::get_firmware_revision()?
    );
    println!("Board serial: 0x{:x}", mailbox::get_board_serial()?);

    loop {
        let c = uart::receive()?;
        uart::send(c)?;
    }
}
