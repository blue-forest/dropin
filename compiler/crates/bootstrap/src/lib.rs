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

use dropin_core::print_to;

pub mod path;
use path::{get_model_path, get_recipe, read_file};
pub mod expressions;
use expressions::Expression;
pub mod modules;
use modules::Compiler;
pub mod syntaxes;
use syntaxes::Patterns;
pub mod sys;
pub mod utils;
use sys::{Args, WasiUnwrap};

pub struct Recipe<'syntax, 'recipe> {
	pub syntax: &'syntax str,
	pub patterns: Patterns<'syntax>,
	pub recipe: &'recipe str,
	pub expression: Expression<'syntax, 'recipe>,
}

impl<'syntax, 'recipe> Recipe<'syntax, 'recipe> {
	pub fn new(syntax: &'syntax str, recipe: &'recipe str) -> Self {
		let patterns = Patterns::new(syntax);
		let expression = patterns.parse(recipe).wasi_unwrap();
		Self {
			syntax,
			patterns,
			recipe,
			expression,
		}
	}
}

const SYNTAX_MODELS: &str = "blueforest:dropin-modules:v1:Models";
const SYNTAX_MODULES: &str = "blueforest:dropin-modules:v1:Automations/Modules";

#[no_mangle]
pub fn _start() {
	let args = Args::new();
	if args.len() != 2 {
		print_to("expected argument: <model>", 2);
		unsafe { wasi::proc_exit(1) };
	}
	let syntax_models_content = &get_recipe("syntaxes", SYNTAX_MODELS);
	let syntax_modules_content = &get_recipe("syntaxes", SYNTAX_MODULES);

	let model_path = get_model_path(args.get(1).wasi_unwrap());
	let mut model_recipe_path = model_path.parent().wasi_unwrap().to_path_buf();
	model_recipe_path.push(".dropin");
	let model_content = read_file(&model_recipe_path);
	let model_recipe = Recipe::new(syntax_models_content, &model_content);

	let module = model_recipe.expression.iter().next().wasi_unwrap();
	let module_id = module.iter().next().unwrap();

	let module_content = &get_recipe("modules", module_id.as_str());
	let module_recipe = Recipe::new(syntax_modules_content, module_content);
	let compiler = Compiler::new(module_recipe);
	let binary = compiler.compile(&model_path).unwrap().finish();
	let data = [wasi::Ciovec {
		buf: binary.as_ptr(),
		buf_len: binary.len(),
	}];
	unsafe { wasi::fd_write(1, &data).unwrap() };
}
