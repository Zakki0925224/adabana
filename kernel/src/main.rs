#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use addr::VirtualAddress;
use color::ColorCode;
use device_tree::DeviceTree;
use framebuffer::PixelFormat;

mod addr;
mod asm;
mod boot;
mod color;
mod console;
mod cpu;
mod device_tree;
mod draw;
mod error;
mod fdt;
mod font;
mod framebuffer;
mod framebuffer_console;
mod gpio;
mod mailbox;
mod mutex;
mod panic;
mod uart;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    extern "C" {
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    let fdt_addr = asm::read_x0();
    kernel_main2(fdt_addr.into()).unwrap();
    unreachable!();
}

fn kernel_main2(fdt_addr: VirtualAddress) -> error::Result<()> {
    let _device_tree = DeviceTree::new(fdt_addr)?;
    let cpu_model = cpu::detect_cpu_model()?;

    // uart::init()?;
    println!("Starting kernel...");
    println!("FDT addr: {:?}", fdt_addr);
    println!("CPU: {:?}", cpu_model);
    // println!(
    //     "Firmware revision: 0x{:x}",
    //     mailbox::get_firmware_revision()?
    // );

    // let fb_info = mailbox::init_framebuffer((480, 320), (480, 320), 32, PixelFormat::default())?;
    // framebuffer::init(fb_info)?;
    // framebuffer_console::init(ColorCode::GREEN, ColorCode::BLACK)?;
    // println!("Framebuffer: {:?}", fb_info);

    loop {
        let c = match uart::receive() {
            Ok(c) => c,
            Err(_) => {
                continue;
            }
        };
        print!("{}", c);
    }
}
