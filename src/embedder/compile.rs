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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use wasmtime::{Linker, Store};
use wasmtime_wasi::sync::stdio::stdout;
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use std::fs::File;
use std::path::Path;

use dropin_helpers::fs::wasm;

use super::Embedder;

impl Embedder {
	fn compile_ctx(root: &Path, owner: &str, model: &str) -> WasiCtx {
		WasiCtxBuilder::new()
			.stderr(Box::new(stdout()))
			.stdout(Box::new(wasmtime_wasi::sync::file::File::from_cap_std(
				cap_std::fs::File::from_std(
					File::create(wasm(root, owner, model, "v1")).unwrap(),
				),
			)))
			.args(&[
				"dropin-bootstrap_v1.wasm".to_string(),
				format!("{}:{}:v1", owner, model),
			])
			.unwrap()
			.preopened_dir(
				Dir::from_std_file(File::open(root).unwrap()),
				Path::new("/"),
			)
			.unwrap()
			.build()
	}

	pub fn compile(&mut self, root: &Path, owner: &str, model: &str) {
		if self.compile.is_none() {
			let handle = self.compile_handle.take().unwrap();
			self.compile = Some(handle.join().unwrap());
		}
		let module = self.compile.as_ref().unwrap();
		let mut linker = Linker::new(&self.engine);
		wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
		let mut store =
			Store::new(&self.engine, Self::compile_ctx(root, owner, model));
		let instance = linker.instantiate(&mut store, module).unwrap();
		let start = instance
			.get_typed_func::<(), (), _>(&mut store, "_start")
			.unwrap();
		if let Err(err) = start.call(&mut store, ()) {
			println!("{}", err);
		}
	}
}
