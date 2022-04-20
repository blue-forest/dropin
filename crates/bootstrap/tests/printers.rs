use std::env::set_var;
use std::path::PathBuf;

use dropin_bootstrap::modules::compile;
use dropin_bootstrap::path::get_recipe;
use dropin_bootstrap::syntaxes::Patterns;

mod common;
use common::Embedder;

#[test]
fn hello_world() {
  let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  root.push("..");
  root.push("..");
  root.push("test-recipes");
  set_var("DROPIN_ROOT", root.to_str().unwrap());

  let syntax_content = &get_recipe("syntaxes", "message");
  let module_content = &get_recipe("modules",  "printer");
  let recipes_content = &get_recipe("printer", "printer");
  let patterns = Patterns::new(syntax_content);
  let expression = patterns.parse(module_content).unwrap();
  let module = compile(expression).unwrap();
  let wasm = module.finish();

  let embedder = Embedder::new(8080);
}
