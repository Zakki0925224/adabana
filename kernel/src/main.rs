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

    let fb_info = mailbox::init_framebuffer((480, 320), (480, 320), 32, PixelFormat::default())?;
    println!("Framebuffer: {:?}", fb_info);
    framebuffer::init(fb_info)?;
    framebuffer::fill(ColorCode::new(255, 255, 255))?;
    framebuffer::draw_rect(100, 100, 50, 50, ColorCode::new(255, 0, 0))?;
    framebuffer::draw_rect(125, 215, 100, 100, ColorCode::new(0, 255, 0))?;
    framebuffer::draw_rect(370, 10, 100, 100, ColorCode::new(0, 0, 255))?;
    framebuffer::draw_string(
        0,
        0,
        "Hello, world!",
        ColorCode::new(255, 255, 255),
        ColorCode::new(0, 0, 0),
    )?;

    loop {
        let c = uart::receive()?;
        uart::send(c)?;
    }
}
