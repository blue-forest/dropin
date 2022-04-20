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
