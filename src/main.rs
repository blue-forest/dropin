/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation under version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

mod interactive;

fn main() {
  let mut args = std::env::args();
  if args.len() > 1 {
    let arg = args.nth(1).unwrap();
    use wasmtime::*;
    use wasmtime_wasi::*;
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let wasi_ctx = WasiCtxBuilder::new()
      .inherit_stdio()
      .args(&[arg.to_string()]).unwrap()
      .build();
    let std_binary: &[u8] = include_bytes!(concat!(
      env!("OUT_DIR"), "/dropin_modules.wasm",
    ));
    let mut store = Store::new(&engine, wasi_ctx);
    let std = Module::from_binary(&engine, std_binary).unwrap();
    let std_instance = linker.instantiate(&mut store, &std).unwrap();
    linker.instance(
      &mut store, "blueforest:dropin-std:v1", std_instance,
    ).unwrap();
    let main = Module::from_file(&engine, arg).unwrap();
    let main_instance = linker.instantiate(&mut store, &main).unwrap();
    let start = main_instance.get_typed_func::<(), (), _>(
      &mut store, "_start"
    ).unwrap();
    start.call(&mut store, ()).unwrap();
    return;
  }
  interactive::Cli::new().run();
}

