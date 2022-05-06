use wasmtime::{Linker, Module, Store};
use wasmtime_wasi::{self, WasiCtx, WasiCtxBuilder};
use wasmtime_wasi::sync::Dir;

use std::fs::{File};
use std::path::Path;

use super::Embedder;

impl Embedder {
  fn run_ctx(root: Option<&Path>) -> WasiCtx {
    let mut builder = WasiCtxBuilder::new().inherit_stdio();
    if let Some(root) = root {
      builder = builder.preopened_dir(
        Dir::from_std_file(File::open(root).unwrap()),
        Path::new("/"),
      ).unwrap();
    }
    builder.build()
  }

  pub fn run(&self, root: Option<&Path>, path: &Path) {
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
