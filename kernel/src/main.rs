#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use addr::VirtualAddress;
use device_tree::DeviceTree;
use mailbox::{Channel, Mailbox, Tag, TagId, TagStatus};

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

    let mut mbox = Mailbox::new();
    let mut tag: Tag<10> = Tag::new(TagId::HardwareGetBoardSerial, TagStatus::Request);
    let tag_s = tag.slice_mut();
    tag_s[2] = 8; // buffer size
    tag_s[3] = 8;
    tag_s[4] = 0; // output buffer
    tag_s[5] = 0;
    tag_s[6] = mailbox::TAG_LAST; // last
    let offset = mbox.write_tag(tag.slice())?;
    mbox.call(Channel::PropertyTags)?;
    println!("response: {:?}", &mbox.inner_slice()[offset..offset + 10]);

    loop {
        let c = uart::receive()?;
        uart::send(c)?;
    }
}
