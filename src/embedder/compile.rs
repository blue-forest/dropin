use cap_std;
use wasmtime::{Linker, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::sync::stdio::stdout;

use std::path::Path;
use std::fs::File;

use dropin_utils::path::get_build;

use super::Embedder;

impl Embedder {
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
}
