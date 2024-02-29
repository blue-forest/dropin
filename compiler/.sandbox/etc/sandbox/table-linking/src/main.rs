/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2024 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() {
	let engine = Engine::default();
	let mut linker = Linker::new(&engine);
	wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
	let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
	let mut store = Store::new(&engine, wasi_ctx);

	let imported = Module::from_file(&engine, "imported.wat").unwrap();
	let instance = linker.instantiate(&mut store, &imported).unwrap();

	let table = instance.get_export(&mut store, "table").unwrap();
	linker.define("env", "table", table).unwrap();

	let main = Module::from_file(&engine, "main.wat").unwrap();
	let instance2 = linker.instantiate(&mut store, &main).unwrap();

	let start = instance2
		.get_typed_func::<(), (), _>(&mut store, "_start")
		.unwrap();
	start.call(&mut store, ()).unwrap();
}
