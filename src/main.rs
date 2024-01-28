#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jimos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod vga_buffer;
pub mod serial;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

use bootloader::{BootInfo, entry_point};
use jimos::{allocator, memory::{self, BootInfoFrameAllocator}};
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

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap init fail");

    let x = Box::new(41);

    println!("heap value at {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    println!("vec at {:p}", vec.as_slice());

    let ref_cnted = Rc::new(vec![1, 2, 3]);
    let cloned_ref = ref_cnted.clone();
    println!("current ref count is {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(ref_cnted);
    println!("current ref count is {}", Rc::strong_count(&cloned_ref));

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
