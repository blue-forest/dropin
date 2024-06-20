#![no_std]

extern crate alloc;

#[global_allocator]
static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;

use alloc::{boxed::Box, collections::BTreeMap, string::String};
use dlmalloc::GlobalDlmalloc;
use dropin_compiler_recipes::ir::Model;
use gen::Gen;
use prost::Message;

use crate::{stage::Stage, stage0::Stage0, stage1::Stage1, visit::Visit};

pub const EXTENSION: &str = ".dart";

trait Stated<S> {
  fn state(&self) -> &S;
}

mod dependencies;
mod formats;
mod gen;
mod imports;
mod objects_getter;
mod properties_resolver;
mod stage;
mod stage0;
mod stage1;
mod updated_listeners;
mod visit;

#[no_mangle]
pub fn codegen(protobuf: *mut [u8]) -> *mut BTreeMap<String, String> {
  let protobuf = unsafe { Box::from_raw(protobuf) };
  let model = Model::decode(protobuf.as_ref()).unwrap();
  let stage0 = Stage::new(Stage0::default()).build(&model);
  let stage1 = Stage::new(Stage1::new(&stage0)).build(&model);
  let gen = Gen::new(&stage1);
  Box::into_raw(Box::new(gen.gen(&model).unwrap()))
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
