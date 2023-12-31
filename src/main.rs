#![no_std] // don't link the rust sta library
#![no_main]// disable all rust entry points

use core::panic::PanicInfo;
static HELLO :&[u8] = b"Rust-OS, An OS built on Rust";
#[no_mangle] // don't mangle this function name
pub extern "C" fn _start() -> ! {

    //this function is the entry point since the 
    //linker looks for a function _start() by default
    //

    let vga_buffer = 0xb8000 as *mut u8; // declares a raw pointer for the vga_buffer
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize *2) = byte; // writes each byte HELLO to vga buffer
            *vga_buffer.offset(i as isize *2 + 1) = 0xb; // writes to the attribute section for
                                                         // each character in vga buffer
        }
    }
    loop{}
}

#[panic_handler]
//this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


