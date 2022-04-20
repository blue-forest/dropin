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

use structopt::StructOpt;

use std::fmt::Debug;
use std::fs::write;

use dropin_utils::path::get_root;
use dropin_bootstrap::Recipe;
use dropin_bootstrap::modules::Compiler;
use dropin_bootstrap::path::get_recipe;

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in bootstrap")]
struct Cli {
  #[structopt(long, short)]
  syntax: String,
  #[structopt(long, short)]
  module: String,
  #[structopt(long, short)]
  collection: String,
  #[structopt(long, short)]
  recipe: String,
}

fn main() {
  let cli = Cli::from_args();
  let root = get_root();
  let syntax_content = &get_recipe(&root, "syntaxes", &cli.syntax);
  let module_content = &get_recipe(&root, "modules", &cli.module);
  let recipe_content = &get_recipe(&root, &cli.collection, &cli.recipe);
  let compiler = Compiler::new(Recipe::new(syntax_content, module_content));
  let recipe_syntax = compiler.get_syntax(&root);
  let recipe = Recipe::new(&recipe_syntax, recipe_content);
  let module = compiler.compile(&root, recipe).unwrap();
  let wasm = module.finish();
  write("module.wasm", wasm).unwrap();
}
