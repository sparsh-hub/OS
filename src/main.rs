// #![no_std] // unlink the Rust standard library
// #![no_main] // disable main entry points

// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]


// #![reexport_test_harness_main = "test_main"]

// use core::panic::PanicInfo;

// mod vga_buffer;
// mod serial;


// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) { // new
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run(); // new
//     }
//     exit_qemu(QemuExitCode::Success);
// }


// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     println!("Hello World{}", "!");

//     #[cfg(test)]
//     test_main();
//     loop {}
// }

// #[test_case]
// fn trivial_assertion() {
//     assert_eq!(1, 1);
// }

// // our existing panic handler
// #[cfg(not(test))] // new attribute
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }

// // our panic handler in test mode
// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     serial_println!("[failed]\n");
//     serial_println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);
//     loop {}
// }





// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }

// pub trait Testable {
//     fn run(&self) -> ();
// }

// impl<T> Testable for T
// where
//     T: Fn(),
// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(try_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use try_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    try_os::init(); // new

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    try_os::test_panic_handler(info)
}
