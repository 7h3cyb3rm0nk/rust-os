#![no_std] // don't link the rust sta library
#![no_main]// disable all rust entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle this function name
pub extern "C" fn _start() -> ! {
    //this function is the entry point since the 
    //linker looks for a function _start() by default
    loop{}
}

#[panic_handler]
//this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


