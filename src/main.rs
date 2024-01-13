#![no_std] // don't link the rust sta library
#![no_main]// disable all rust entry points
#![allow(dead_code)]


#![reexport_test_harness_main = "test_main"] // using this attribute to reexport the testmain
                                             // function as function that runs the tests


#![feature(custom_test_frameworks)] //using custom_test_framework for testing
#![test_runner(crate::test_runner)] // using the test_runner fn to run the test
use core::panic::PanicInfo;
mod vga_buffer;
 static HELLO :&[u8] = b"Rust-OS, An OS built on Rust";
#[no_mangle] // don't mangle this function name
             //
             //
pub extern "C" fn _start() -> ! {

    //this function is the entry point since the 
    //linker looks for a function _start() by default
    //
    //
    // let vga_buffer = 0xb8000 as *mut u8; // declares a raw pointer for the vga_buffer
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize *2) = byte; // writes each byte HELLO to vga buffer
    //         *vga_buffer.offset(i as isize *2 + 1) = 0xb; // writes to the attribute section for
    //                                                      // each character in vga buffer
    //     }
    // }
    //
    // vga_buffer::WRITER.lock().write_string("Hello World");
    // write!(vga_buffer::WRITER.lock(), "Rust Os").unwrap();
    //
    
    //
    println!("RUST-OS VERSION 0.1.0 ");
    #[cfg(test)]
    test_main();
    loop{}
}

#[panic_handler]
//this function is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//tests
// this function takes all the tests marked with #[test_case]
// and does the testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) { 
    println!("Running {} tests", tests.len()); 
    for test in tests {
        test();
    }
}


#[test_case]
fn trivial_assertion() {
    println!("trivial assertion");
    assert_eq!(1,1);
    println!("[ok]");
}
