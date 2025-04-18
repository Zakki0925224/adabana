use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info.message());
    println!("{:?}", info.location());

    loop {}
}
