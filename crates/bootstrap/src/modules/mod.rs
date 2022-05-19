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

use wasm_encoder::Module;

use std::path::Path;

use dropin_helpers::fs::read_recipe;
use dropin_helpers::{decompose_recipe, Header};

use crate::{Recipe, WasiUnwrap};

mod builder;
use builder::{Core, ModuleBuilder};

mod error;
pub use error::CompileError;

mod functions;
pub use functions::FunctionState;

mod value;
pub(self) use value::Value;

struct State<'a> {
	std: Core<'a>,
}

pub struct Compiler<'syntax, 'module> {
	module: Recipe<'syntax, 'module>,
}

impl<'syntax, 'module> Compiler<'syntax, 'module> {
	pub fn new(module: Recipe<'syntax, 'module>) -> Self {
		Self { module }
	}

	pub fn compile(
		&self,
		_path: &Path,
	) -> Result<(Module, Header<'module>), CompileError> {
		let mut builder = ModuleBuilder::default();
		let mut state = State {
			std: Core::default(),
		};
		let mut item = Header::default();

		let mut iter = self.module.expression.iter();
		iter.next(); // skip recipes
		let mut function_iter = iter.next().wasi_unwrap().iter();
		let mut function_state = FunctionState::default();
		let mut function = self.fn_profile(
			&mut item,
			&mut builder,
			&mut function_state,
			&mut function_iter,
		);
		self.instructions(
			&mut builder,
			&mut state,
			&function_state,
			&mut function,
			&function_iter.next().wasi_unwrap(),
		);
		builder.function(function);

		Ok((builder.build(), item))
	}

	pub fn get_syntax(&self) -> String {
		let id = self.module.expression.iter().next().wasi_unwrap().as_str();
		let (owner, model, version, recipe) = decompose_recipe(id);
		read_recipe(&Path::new("/"), owner, model, version, "syntaxes", recipe)
	}
}
