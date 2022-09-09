#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(custom_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use custom_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    custom_os::test_panic_handler(_info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello kernel world!\n");

    custom_os::init();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}
