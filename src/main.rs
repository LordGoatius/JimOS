#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jimos::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;
pub mod serial;

use bootloader::{BootInfo, entry_point};
use jimos::memory::{self, BootInfoFrameAllocator};
use core::panic::PanicInfo;
use x86_64::{structures::paging::{Page, Translate}, VirtAddr};

entry_point!(kernel_main);

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Got it :{}:", "sunglasses");

    jimos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let pafe_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { pafe_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    jimos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    jimos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jimos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
