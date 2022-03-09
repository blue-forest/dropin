use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime::*;

fn main() {
  let engine = Engine::default();
  let mut linker = Linker::new(&engine);
  wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
  let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
  let mut store = Store::new(&engine, wasi_ctx);

  let test = Module::from_file(&engine, "test.wat").unwrap();
  let instance = linker.instantiate(&mut store, &test).unwrap();
  /*
  let start = instance.get_typed_func::<(), (), _>(
    &mut store, "_start"
  ).unwrap();
  start.call(&mut store, ()).unwrap();
  */

  let table = instance.get_export(&mut store, "table").unwrap();
  linker.define("test", "table", table).unwrap();

  let test2 = Module::from_file(&engine, "test2.wat").unwrap();
  let instance2 = linker.instantiate(&mut store, &test2).unwrap();

  let start = instance2.get_typed_func::<(), (), _>(
    &mut store, "_start"
  ).unwrap();
  start.call(&mut store, ()).unwrap();
}
