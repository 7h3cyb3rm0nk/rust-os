#![no_std] // don't link the rust sta library
#![no_main]// disable all rust entry points
#![allow(dead_code)]


#![reexport_test_harness_main = "test_main"] // using this attribute to reexport the testmain
                                             // function as function that runs the tests


#![feature(custom_test_frameworks)] //using custom_test_framework for testing
#![test_runner(crate::test_runner)] // using the test_runner fn to run the test
use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

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
    serial_println!("[failed]");
    serial_println!("Error {}", info);
    exit_qemu(QemuExitCode::Failed);
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



pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where T: Fn()
{
    fn run(&self) {
        serial_println!("{}...\t",core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }

}
//tests
// this function takes all the tests marked with #[test_case]
// and does the testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) { 
    serial_println!("Running {} tests", tests.len()); 
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}
