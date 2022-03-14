/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
