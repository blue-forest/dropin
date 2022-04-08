use structopt::StructOpt;

use std::fmt::Debug;

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
  println!("PATTERNS:    {:?}", patterns);
  let expression = patterns.parse(module_content);
  println!("MODULE:      {:?}", module_content);
  println!("EXPRESSIONS: {:?}", expression);
}
