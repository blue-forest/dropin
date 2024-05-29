#![no_std]

extern crate alloc;

#[global_allocator]
static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;

use alloc::{boxed::Box, ffi::CString};
use dlmalloc::GlobalDlmalloc;
use dropin_compiler_recipes::ir::Component;
use prost::Message;

use self::component::GenComponent;

mod component;
mod expression;
mod format;
mod keys;

#[no_mangle]
pub fn codegen(protobuf: *mut [u8]) -> CString {
  let protobuf = unsafe { Box::from_raw(protobuf) };
  let component = Component::decode(protobuf.as_ref()).unwrap();
  let gen = GenComponent::from(component);
  CString::new(gen.gen().unwrap()).unwrap()
}

// #[cfg(debug_assertions)]
// use lazy_static::lazy_static;
// #[cfg(debug_assertions)]
// use wasi::cli::stdout::OutputStream;

// #[cfg(debug_assertions)]
// lazy_static! {
//   static ref STDOUT: OutputStream = wasi::cli::stdout::get_stdout();
// }

// #[cfg(debug_assertions)]
// struct Printer;

// #[cfg(debug_assertions)]
// impl core::fmt::Write for Printer {
//   fn write_str(&mut self, s: &str) -> core::fmt::Result {
//     crate::STDOUT.write(s.as_bytes()).unwrap();
//     Ok(())
//   }
// }
