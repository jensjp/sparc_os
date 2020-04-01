#![no_std]
#![no_main]
use core::panic::PanicInfo;

//static HELLO : &[u8] = b"Hello World !";

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    /*let vga_base = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe{
            *vga_base.offset(i as isize * 2) = byte;
            *vga_base.offset(i as isize * 2 + 1) = 0xb;
        }
    }*/
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
