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

use wasmtime::*;

use std::fs::{read, write};
use std::path::Path;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

use dropin_helpers::fs::wasm;
use dropin_pm::fetch;

mod compile;
mod run;
pub use run::Param;

pub struct Embedder {
	pub engine: Arc<Engine>,
	pub core: Option<Module>,
	pub core_handle: Option<JoinHandle<Module>>,
	pub compile: Option<Module>,
	pub compile_handle: Option<JoinHandle<Module>>,
}

impl Embedder {
	fn fetch<'a>(
		root: &Path,
		engine: Arc<Engine>,
		model: &'a str,
	) -> impl FnMut() -> Module + 'a {
		let path = wasm(root, "blueforest", model, "v1");
		move || {
			let binary = if !path.exists() {
				let binary = fetch("blueforest", model, "v1").unwrap();
				write(&path, &binary).unwrap();
				binary
			} else {
				read(&path).unwrap().into()
			};
			Module::new(&engine, binary).unwrap()
		}
	}

	pub fn new(root: &Path) -> Self {
		let engine = Arc::new(
			Engine::new(
				Config::new()
					.debug_info(true)
					.wasm_backtrace_details(WasmBacktraceDetails::Enable),
			)
			.unwrap(),
		);
		let core_handle =
			Some(spawn(Embedder::fetch(root, engine.clone(), "dropin-core")));
		let compile_handle = Some(spawn(Embedder::fetch(
			root,
			engine.clone(),
			"dropin-bootstrap",
		)));

		Self {
			engine,
			core: None,
			core_handle,
			compile: None,
			compile_handle,
		}
	}
}
