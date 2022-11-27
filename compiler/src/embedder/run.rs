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

use wasmtime::{Linker, Module, Store};
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::{self, WasiCtx, WasiCtxBuilder};

use std::fs::File;
use std::path::Path;

use super::Embedder;

impl Embedder {
	fn run_ctx(root: Option<&Path>) -> WasiCtx {
		let mut builder = WasiCtxBuilder::new().inherit_stdio();
		if let Some(root) = root {
			builder = builder
				.preopened_dir(
					Dir::from_std_file(File::open(root).unwrap()),
					Path::new("/"),
				)
				.unwrap();
		}
		builder.build()
	}

	pub fn run(&mut self, root: Option<&Path>, path: &Path) {
		if self.core.is_none() {
			let handle = self.core_handle.take().unwrap();
			self.core = Some(handle.join().unwrap());
		}
		let module = self.core.as_ref().unwrap();
		let mut linker = Linker::new(&self.engine);
		wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
		let mut store = Store::new(&self.engine, Self::run_ctx(root));
		let core_instance = linker.instantiate(&mut store, module).unwrap();
		linker
			.instance(&mut store, "blueforest:dropin-core:v1", core_instance)
			.unwrap();
		let main = Module::from_file(&self.engine, path).unwrap();
		let main_instance = linker.instantiate(&mut store, &main).unwrap();
		let start = main_instance
			.get_typed_func::<(), (), _>(&mut store, "_start")
			.unwrap();
		start.call(&mut store, ()).unwrap();
	}
}
