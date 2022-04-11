use pretty_hex::{config_hex, HexConfig};
use structopt::StructOpt;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime::*;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
enum Commands {
  Memory{
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    #[structopt(long, short, default_value = "0")]
    start: usize,
    #[structopt(long, short, default_value = "128")]
    len: usize,
  }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "drop'in debugger")]
pub struct Cli {
  #[structopt(subcommand)]
  cmd: Commands,
}

fn main() {
  let cli = Cli::from_args();
  match cli.cmd {
    Commands::Memory{ file, start, len } => memory(file, start, len)
  }
}

fn memory(file: PathBuf, start: usize, len: usize) {
  let engine = Engine::default();
  let mut linker = Linker::new(&engine);
  wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
  let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
  let mut store = Store::new(&engine, wasi_ctx);

  let module = Module::from_file(&engine, file).unwrap();
  let instance = linker.instantiate(&mut store, &module).unwrap();

  let start_fn = instance.get_typed_func::<(), (), _>(
    &mut store, "_start"
  ).unwrap();
  start_fn.call(&mut store, ()).unwrap();
  let memory = if let Extern::Memory(memory) = instance.get_export(
    &mut store, "memory",
  ).unwrap() {
    memory
  } else {
    panic!("exported member \"memory\" is not Memory");
  };
  let data = memory.data(&store).get(start..start+len).unwrap();

  let cfg = HexConfig {
    title: false,
    ..HexConfig::default()
  };
  println!("         0  1  2  3   4  5  6  7   8  9  a  b   c  d  e  f");
  println!("{}", config_hex(&data, cfg));
}
