use structopt::StructOpt;

use std::fmt::Debug;
use std::fs::write;

mod expressions;

mod modules;
use modules::compile;

mod path;
use path::get_recipe;

mod syntaxes;
use syntaxes::Patterns;


#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in bootstrap")]
struct Cli {
  #[structopt(long, short)]
  syntax: String,
  #[structopt(long, short)]
  module: String,
}

fn main() {
  let cli = Cli::from_args();
  let syntax_content = &get_recipe("syntaxes", cli.syntax);
  let module_content = &get_recipe("modules", cli.module);
  let patterns = Patterns::new(syntax_content);
  let expression = patterns.parse(module_content).unwrap();
  let module = compile(expression).unwrap();
  let wasm = module.finish();
  write("module.wasm", wasm).unwrap();
}
