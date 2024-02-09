#![no_std] // don't link the rust sta library
#![no_main]// disable all rust entry points
#![allow(dead_code)]


#![reexport_test_harness_main = "test_main"] // using this attribute to reexport the testmain
                                             // function as function that runs the tests


#![feature(custom_test_frameworks)] //using custom_test_framework for testing
#![test_runner(blog-os::test_runner)] // using the test_runner fn to run the test
use core::panic::PanicInfo;
use rust-os::println;


#[no_mangle] // don't mangle this function name
pub extern "C" fn _start() -> ! {

    println!("RUST-OS VERSION 0.1.0 ");
    #[cfg(test)]
    test_main();
    loop{}
}


#[cfg(not(test))]
#[panic_handler]
//this function is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}



#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust-os::test_panic_handler(info)
    loop {}
}


