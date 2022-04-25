use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime::{ Engine, Linker, Module, Store };

// https://github.com/rust-lang/rust/issues/75075
#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {() => ( r"\")}

#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {() => ( r"/")}

static STD_BINARY: &[u8] = include_bytes!(concat!(
  env!("OUT_DIR"), PATH_SEPARATOR!(), "dropin_modules.wasm",
));

use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use super::{Cli, Command, Recipe};
use super::select::Selection;

pub struct Run(Arc<Selection>);

impl Run {
  pub fn new(selection: Arc<Selection>) -> Self {
    Self(selection)
  }
}

impl Display for Run {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "run".fmt(f)
  }
}

impl Command for Run {
  fn run(&self, cli: &mut Cli) -> u32 {
    todo!()
    /* TODO
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);

    let std = Module::from_binary(&engine, std_binary)?;
    let std_instance = linker.instantiate(&mut store, &std)?;
    linker.instance(&mut store, "blueforest:std:v1", std_instance)?;

    let main = Module::from_file(&engine, "main.wat")?;
    let main_instance = linker.instantiate(&mut store, &main)?;

    let start = main_instance.get_typed_func::<(), (), _>(
      &mut store, "_start"
    )?;
    start.call(&mut store, ())?;
    0
    */
  }
}

