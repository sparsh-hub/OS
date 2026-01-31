#![no_std] // unlink the Rust standard library
#![no_main] // disable main entry points

use core::panic::PanicInfo;

#[unsafe(no_mangle)] 
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

