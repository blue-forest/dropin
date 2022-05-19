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
use dropin_helpers::fs::{
	self, header, model_path, read, read_recipe, wasm, write,
};
use dropin_helpers::{decompose_recipe, decompose_version};

mod expressions;
pub use expressions::Expression;
pub mod modules;
pub use modules::Compiler;
pub mod syntaxes;
use syntaxes::Patterns;
pub mod sys;
use sys::{Args, WasiUnwrap};
pub mod utils;

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

const OWNER: &str = "blueforest";
const DROPIN_MODULES: &str = "dropin-modules";
const MODELS: &str = "Models";
const MODULES: &str = "Automations/Modules";

#[no_mangle]
pub fn _start() {
	let args = Args::new();
	if args.len() != 2 {
		print_to("expected argument: <model>", 2);
		unsafe { wasi::proc_exit(1) };
	}
	let root = fs::root();
	let syntax_models_content =
		&read_recipe(&root, OWNER, DROPIN_MODULES, "v1", "syntaxes", MODELS);
	let syntax_modules_content =
		&read_recipe(&root, OWNER, DROPIN_MODULES, "v1", "syntaxes", MODULES);

	let model_full_id = args.get(1).wasi_unwrap();
	let (model_owner, model_id, model_version) =
		decompose_version(model_full_id);
	let model_path = model_path(&root, model_owner, model_id, model_version);
	let mut model_recipe_path = model_path.parent().wasi_unwrap().to_path_buf();
	model_recipe_path.push(".dropin");
	let model_content = read(&model_recipe_path);
	let model_recipe = Recipe::new(syntax_models_content, &model_content);

	let module = model_recipe.expression.iter().next().wasi_unwrap();
	let module_id = module.iter().next().unwrap();

	let (module_owner, module_model, module_version, module_recipe) =
		decompose_recipe(module_id.as_str());
	let module_content = read_recipe(
		&root,
		module_owner,
		module_model,
		module_version,
		"modules",
		module_recipe,
	);
	let module_recipe = Recipe::new(syntax_modules_content, &module_content);
	let compiler = Compiler::new(module_recipe);
	let (module, item) = compiler.compile(&model_path).unwrap();

	let wasm_binary = module.finish();
	write(
		&wasm(&root, module_owner, module_model, module_version),
		wasm_binary.as_slice(),
	);

	let item_binary = item.to_le_bytes();
	write(
		&header(&root, module_owner, module_model, module_version),
		item_binary.as_slice(),
	);
}
