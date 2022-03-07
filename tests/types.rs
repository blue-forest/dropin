use wasm_ir::Compilable;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime::{Engine, Linker, Module, Store};

use dropin::parser::read_type;

#[test]
fn simple_text() {
  let type_ = read_type(vec![
    "types blueforest:tests:v1:hello_world",
    "======================================",
    "templates",
    "  '' text",
  ].join("\n"));
  let module = type_.compile();
  let binary = module.compile();
  let engine = Engine::default();
  let module = Module::new(&engine, binary).unwrap();
  let mut linker = Linker::new(&engine);

  wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
  let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
  let mut store = Store::new(&engine, wasi_ctx);

  let _instance = linker.instantiate(&mut store, &module).unwrap();

  /* TODO
  let validate = instance.get_typed_func::<(), u32, _>(
    &mut store, "validate"
  ).unwrap();
  validate.call(&mut store, ()).unwrap();
  */
}
