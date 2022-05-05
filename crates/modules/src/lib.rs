#![no_std]

extern crate wee_alloc;

use core::alloc::{GlobalAlloc, Layout};

const NEW_LINE: &str = "\n";

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub fn print(message: &str) {
  print_to(message, 1)
}

pub fn print_to(message: &str, fd: u32) {
  let data = [
    wasi::Ciovec {
      buf: message.as_ptr(),
      buf_len: message.len(),
    },
    wasi::Ciovec {
      buf: NEW_LINE.as_ptr(),
      buf_len: NEW_LINE.len(),
    },
  ];
  unsafe { wasi::fd_write(fd, &data).unwrap() };
}

#[no_mangle]
pub fn alloc(size: usize, align: usize) -> *mut u8 {
  unsafe { ALLOC.alloc(Layout::from_size_align(size, align).unwrap()) }
}
