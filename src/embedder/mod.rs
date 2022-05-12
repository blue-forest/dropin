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

use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use wasmtime::*;

mod compile;
mod run;

// https://github.com/rust-lang/rust/issues/75075
#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {
    () => {
        r"\"
    };
}

#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {
    () => {
        r"/"
    };
}

static CORE_BINARY: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    PATH_SEPARATOR!(),
    "dropin-core_v1.wasm",
));

static BOOTSTRAP_BINARY: &[u8] = include_bytes!(concat!(
    env!("OUT_DIR"),
    PATH_SEPARATOR!(),
    "dropin-bootstrap_v1.wasm",
));

pub struct Embedder {
    pub engine: Arc<Engine>,
    pub core: Module,
    pub compile_module: Option<Module>,
    pub compile_module_handle: Option<JoinHandle<Module>>,
}

impl Default for Embedder {
    fn default() -> Self {
        let engine = Arc::new(Engine::default());

        let core = Module::new(&engine, CORE_BINARY).unwrap();

        let compile_module = None;
        let engine_clone = engine.clone();
        let compile_module_handle = Some(spawn(move || {
            Module::new(&engine_clone, BOOTSTRAP_BINARY).unwrap()
        }));

        Self {
            engine,
            core,
            compile_module,
            compile_module_handle,
        }
    }
}
