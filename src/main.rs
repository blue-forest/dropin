use structopt::StructOpt;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime::*;

use std::path::PathBuf;
use std::fs::read_to_string;

use dropin_compiler::parser::{read_file, print_pairs};

#[derive(StructOpt, Debug)]
enum Commands {
  /// Debug tools. To learn more: dropin debug --help
  Debug {
    #[structopt(subcommand)]
    cmd: DebugTools,
  },
}

#[derive(StructOpt, Debug)]
enum DebugTools {
  /// Print the recipe parser output
  Recipe {
    /// Recipe path
    file: String,
  },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in compiler")]
pub struct Cli {
  #[structopt(subcommand)]
  cmd: Commands,
}

fn main() {
  let cli = Cli::from_args();
  match cli.cmd {
    Commands::Debug{cmd} => debug(cmd),
  }
  /*
  let engine = Engine::default();
  let module = Module::from_file(&engine, "sandbox/gen.wasm").unwrap();
  let mut linker = Linker::new(&engine);

  wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();

  let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
  let mut store = Store::new(&engine, wasi_ctx);

  let instance = linker.instantiate(&mut store, &module).unwrap();

  let start = instance.get_typed_func::<(), (), _>(
    &mut store, "_start"
  ).unwrap();
  start.call(&mut store, ()).unwrap();
  */
}

fn debug(cmd: DebugTools) {
  match cmd {
    DebugTools::Recipe{file} => {
      let content = read_to_string(file).unwrap();
      let pair = read_file(content.as_str());
      print_pairs(pair.into_inner(), 0);
    }
  };
}
