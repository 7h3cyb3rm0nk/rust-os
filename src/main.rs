#![no_std] // don't link the rust sta library
#![no_main]// disable all rust entry points
#![allow(dead_code)]


#![reexport_test_harness_main = "test_main"] // using this attribute to reexport the testmain
                                             // function as function that runs the tests


#![feature(custom_test_frameworks)] //using custom_test_framework for testing
#![test_runner(crate::test_runner)] // using the test_runner fn to run the test
use core::panic::PanicInfo;
mod vga_buffer;


#[no_mangle] // don't mangle this function name
pub extern "C" fn _start() -> ! {

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


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Failed = 0x11,
}
//function to write to the isa-debug-device
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
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
    exit_qemu(QemuExitCode::Success);
}


#[test_case]
fn trivial_assertion() {
    println!("trivial assertion");
    assert_eq!(1,1);
    println!("[ok]");
}
