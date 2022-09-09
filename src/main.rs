#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(custom_os::test_runner)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use custom_os::{hlt_loop, println, serial_println};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use custom_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Initializing...");

    custom_os::init();

    let phys_mem_offset = VirtAddr::new(
        boot_info.physical_memory_offset, //.into_option()
                                          //.expect("physical_memory_offset not initialized"),
    );
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    custom_os::test_panic_handler(_info);
}
