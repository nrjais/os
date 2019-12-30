#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::memory::BootInfoFrameAllocator;
use os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use os::memory;
  use x86_64::VirtAddr;

  println!("Hello World{}", "!");
  os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut _mapper = unsafe { memory::init(phys_mem_offset) };
  let mut _frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  os::hlt_loop();
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
  os::test_panic_handler(info)
}
