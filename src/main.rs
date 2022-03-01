use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime::*;

fn main() {
  let linker = Linker::new(Engine::new());
  let store = Store::new(linker.engine)
}
