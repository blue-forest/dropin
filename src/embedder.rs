use cap_std;
use wasmtime::*;
use wasmtime_wasi::{self, WasiCtx, WasiCtxBuilder};
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::sync::stdio::stdout;

use std::fs::{File};
use std::path::Path;
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

use dropin_utils::path::get_build;

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

impl Embedder {
  fn run_ctx(root: &Path) -> WasiCtx {
    WasiCtxBuilder::new()
      .inherit_stdio()
      .preopened_dir(
        Dir::from_std_file(File::open(root).unwrap()),
        Path::new("/"),
      ).unwrap()
      .build()
  }

  fn compile_ctx(root: &Path, owner: &str, model: &str) -> WasiCtx {
    WasiCtxBuilder::new()
      .stderr(Box::new(stdout()))
      .stdout(Box::new(
        wasmtime_wasi::sync::file::File::from_cap_std(
          cap_std::fs::File::from_std(
            File::create(get_build(root, owner, model)).unwrap()
          )
        )
      ))
      .args(&[
        "dropin_bootstrap.wasm".to_string(),
        format!("{}:{}:v1", owner, model),
      ]).unwrap()
      .preopened_dir(
        Dir::from_std_file(File::open(root).unwrap()),
        Path::new("/"),
      ).unwrap()
      .build()
  }

  pub fn compile(&mut self, root: &Path, owner: &str, model: &str) {
    if let None = self.compile_module {
      let handle = self.compile_module_handle.take().unwrap();
      self.compile_module = Some(handle.join().unwrap());
    }
    let module = self.compile_module.as_ref().unwrap();
    let mut linker = Linker::new(&self.engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let mut store = Store::new(
      &self.engine, Self::compile_ctx(root, owner, model)
    );
    let instance = linker.instantiate(&mut store, module).unwrap();
    let start = instance.get_typed_func::<(), (), _>(
      &mut store, "_start"
    ).unwrap();
    start.call(&mut store, ()).unwrap();
  }

  pub fn run(&self, root: &Path, path: &Path) {
    let mut linker = Linker::new(&self.engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let mut store = Store::new(&self.engine, Self::run_ctx(root));
    let std_instance = linker.instantiate(&mut store, &self.std).unwrap();
    linker.instance(
      &mut store, "blueforest:dropin-std:v1", std_instance,
    ).unwrap();
    let main = Module::from_file(&self.engine, path).unwrap();
    let main_instance = linker.instantiate(&mut store, &main).unwrap();
    let start = main_instance.get_typed_func::<(), (), _>(
      &mut store, "_start"
    ).unwrap();
    start.call(&mut store, ()).unwrap();
  }
}
