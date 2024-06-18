#![no_std]

extern crate alloc;

#[global_allocator]
static GLOBAL: GlobalDlmalloc = GlobalDlmalloc;

use alloc::{boxed::Box, collections::BTreeMap, string::String};
use dlmalloc::GlobalDlmalloc;
use dropin_compiler_recipes::ir::Model;
use prost::Message;

use crate::{
  properties_resolver::PropertiesResolver,
  setters_listeners::SettersAndListeners, stage::Stage, visit::Visit,
};

trait Stated<S> {
  fn state(&self) -> &S;
}

// mod gen;
// mod imports;
mod objects_getter;
mod properties_resolver;
mod setters_listeners;
mod stage;
mod visit;

#[no_mangle]
pub fn codegen(protobuf: *mut [u8]) -> *mut BTreeMap<String, String> {
  let protobuf = unsafe { Box::from_raw(protobuf) };
  let model = Model::decode(protobuf.as_ref()).unwrap();
  let stage = Stage::new(PropertiesResolver::default());
  let resolver = stage.build(&model);
  let stage = Stage::new(SettersAndListeners::new(&resolver));
  let setters = stage.build(&model);
  todo!("{setters:#?}");
  // let objects_getter = ObjectGetter::new(&model);
  // let listeners = Listeners::new(&model);
  // let setters = Setters::new(&model);
  // let imports = Imports::new(&model);
  // let combine = Combine(objects_getter, listeners, setters, imports);
  // let gen = Gen::new(&combine);
  // Box::into_raw(Box::new(gen.gen().unwrap()))
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
