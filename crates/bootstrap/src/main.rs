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

use dropin_bootstrap::modules::compile;
use dropin_bootstrap::path::get_recipe;
use dropin_bootstrap::syntaxes::Patterns;

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

use expressions::Expression;
pub struct RecipeCompiler<'syntax, 'recipe> {
  syntax:      &'syntax str,
  patterns:    Patterns<'syntax>,
  recipe:      &'recipe str,
  expressions: Expression<'syntax, 'recipe>,
}

fn main() {
  let cli = Cli::from_args();
  let syntax_content = &get_recipe("syntaxes", &cli.syntax);
  let module_content = &get_recipe("modules", &cli.module);
  let recipe_content = &get_recipe(&cli.collection, &cli.recipe);
  let patterns = Patterns::new(syntax_content);
  let expression = patterns.parse(module_content).unwrap();
  let module = compile(expression, recipe_content).unwrap();
  let wasm = module.finish();
  write("module.wasm", wasm).unwrap();
}
