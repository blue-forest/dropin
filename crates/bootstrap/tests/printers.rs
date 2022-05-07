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

use std::io::Read;
use std::path::PathBuf;

use dropin_bootstrap::Recipe;
use dropin_bootstrap::modules::Compiler;
use dropin_bootstrap::path::get_recipe;

mod common;
use common::Embedder;

#[test]
fn hello_world() {
  let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  root.push("..");
  root.push("..");
  root.push("recipes");

  let syntax_content = &get_recipe(
    &root, "syntaxes", "blueforest:dropin:v1:Automations/Modules",
  );
  let module_content = &get_recipe(
    &root, "modules",  "blueforest:tests:v1:printers",
  );
  let recipe_content = &get_recipe(
    &root, "printers", "blueforest:tests:v1:hello_world",
  );
  let compiler = Compiler::new(Recipe::new(syntax_content, module_content));
  let recipe = Recipe::new(syntax_content, recipe_content);
  let module = compiler.compile(&root, recipe).unwrap();
  let wasm = module.finish();

  let mut embedder = Embedder::new(8080);
  embedder.run(wasm);
  let mut stream = embedder.listener.incoming().next().unwrap().unwrap();
  let mut buf: [u8; 12] = [0; 12];
  stream.read(&mut buf).unwrap();
  assert_eq!(std::str::from_utf8(&buf).unwrap(), "Hello\nWorld\n");
}
