#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::allocator;
use os::memory;
use os::memory::BootInfoFrameAllocator;
use os::println;
use os::task::keyboard;
use os::task::{executor::Executor, Task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use x86_64::VirtAddr;

  println!("Hello World{}", "!");
  os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
  allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

  #[cfg(test)]
  test_main();

  let mut executor = Executor::new();
  executor.spawn(Task::new(keyboard::print_keypresses()));
  executor.run();
}

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
