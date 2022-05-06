use wasmtime::*;
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

mod compile;
mod run;

// https://github.com/rust-lang/rust/issues/75075
#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {() => ( r"\")}

#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {() => ( r"/")}

static STD_BINARY: &[u8] = include_bytes!(concat!(
  env!("OUT_DIR"), PATH_SEPARATOR!(), "dropin_modules.wasm",
));

static BOOTSTRAP_BINARY: &[u8] = include_bytes!(concat!(
  env!("OUT_DIR"), PATH_SEPARATOR!(), "dropin_bootstrap.wasm",
));

pub struct Embedder {
  pub engine:                Arc<Engine>,
  pub std:                   Module,
  pub compile_module:        Option<Module>,
  pub compile_module_handle: Option<JoinHandle<Module>>,
}

impl Default for Embedder {
  fn default() -> Self {
    let engine = Arc::new(Engine::default());

    let std = Module::new(&engine, STD_BINARY).unwrap();

    let compile_module = None;
    let engine_clone = engine.clone();
    let compile_module_handle = Some(spawn(move ||
      Module::new(&engine_clone, BOOTSTRAP_BINARY).unwrap()
    ));

    Self{ engine, std, compile_module, compile_module_handle }
  }
}
