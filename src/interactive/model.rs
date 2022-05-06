/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation under version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use edit::edit_file;
use wasmtime::{
  Engine,
  Linker,
  Module,
  Store,
};
use wasmtime_wasi::{self, WasiCtxBuilder};
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::sync::stdio::stdout;

use std::fmt::{Display, Formatter, Error};
use std::fs::{create_dir_all, File};
use std::path::Path;

use super::{Cli, Command, get_dirs};
use super::path::{get_build, get_owner};
use super::utils::validate_name;

// https://github.com/rust-lang/rust/issues/75075
#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {() => ( r"\")}

#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {() => ( r"/")}

static STD_BINARY: &[u8] = include_bytes!(concat!(
  env!("OUT_DIR"), PATH_SEPARATOR!(), "dropin_modules.wasm",
));

static BOOTSTRAP_BINARY: &[u8] = include_bytes!(concat!(
  env!("OUT_DIR"), PATH_SEPARATOR!(), "dropin_bootstrap.wasm",
));

pub struct ModelCommand;

impl Display for ModelCommand {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "models".fmt(f)
  }
}

impl Command for ModelCommand {
  fn run(&self, cli: &mut Cli) -> u32 {
    let mut path = get_owner(cli).unwrap();
    path.push("models");
    cli.run_select("Model", |cli| {
      cli.models = get_dirs(&path);
      let mut commands: Vec<Box<dyn Command>> = Vec::new();
      for (i, model) in cli.models.iter().enumerate() {
        commands.push(Box::new(Model{
          name:  model.to_string(),
          index: i,
        }));
      }
      commands.push(Box::new(Add{}));
      commands
    })
  }

  fn is_enabled(&self, cli: &Cli) -> bool { cli.owner_selected.is_some() }
}

struct Model {
  name:  String,
  index: usize,
}

impl Display for Model {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    self.name.fmt(f)
  }
}

impl Command for Model {
  fn run(&self, cli: &mut Cli) -> u32 {
    cli.run_select(&format!("Model {}", self.name), |cli| {
      let mut result: Vec<Box<dyn Command>> = vec![
        Box::new(Select{ name: self.name.clone(), index: self.index }),
        Box::new(Edit{}),
        Box::new(Compile{}),
      ];
      let build_path = get_build(cli);
      if build_path.exists() {
        result.push(Box::new(Run{}));
      }
      result
    })
  }
}

struct Select {
  name:  String,
  index: usize,
}

impl Display for Select {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "select".fmt(f)
  }
}

impl Command for Select {
  fn run(&self, cli: &mut Cli) -> u32 {
    cli.model_selected = Some(self.index);
    cli.config.set_model(self.name.clone());
    2
  }
}

struct Edit;

impl Display for Edit {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "edit".fmt(f)
  }
}

impl Command for Edit {
  fn run(&self, cli: &mut Cli) -> u32 {
    edit_file(&cli.cwd.parent().unwrap().join(".dropin")).unwrap();
    0
  }
}

struct Add;

impl Display for Add {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "add".fmt(f)
  }
}

impl Command for Add {
  fn run(&self, cli: &mut Cli) -> u32 {
    let model_name = loop {
      let model_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Model name for your recipes ? (leave empty to cancel)")
        .allow_empty(true)
        .interact_text().unwrap();
      if model_name.is_empty() { return 0; }
      if let Err(err) = validate_name(&cli.cwd, &model_name) {
        println!("{}", err);
        continue;
      }
      cli.cwd = get_owner(cli).unwrap();
      cli.cwd.push("models");
      cli.cwd.push(&model_name);
      break model_name;
    };
    cli.cwd.push("v1");
    create_dir_all(&cli.cwd).unwrap();
    println!("Model {} created", model_name);
    let index = cli.models.len();
    cli.models.push(model_name);
    cli.model_selected = Some(index);
    cli.config.set_model(cli.models[index].clone());
    1
  }
}

struct Compile;

impl Display for Compile {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "compile".fmt(f)
  }
}

impl Command for Compile {
  fn run(&self, cli: &mut Cli) -> u32 {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let owner = &cli.owners[cli.owner_selected.unwrap()];
    let model = &cli.models[cli.model_selected.unwrap()];
    let wasi_ctx = WasiCtxBuilder::new()
      .stderr(Box::new(stdout()))
      .stdout(Box::new(
        wasmtime_wasi::sync::file::File::from_cap_std(
          cap_std::fs::File::from_std(
            File::create(get_build(cli)).unwrap()
          )
        )
      ))
      .args(&[
        "dropin_bootstrap.wasm".to_string(),
        format!("{}:{}:v1", owner, model),
      ]).unwrap()
      .preopened_dir(
        Dir::from_std_file(File::open(&cli.root).unwrap()),
        Path::new("/"),
      ).unwrap()
      .build();
    let mut store = Store::new(&engine, wasi_ctx);
    let main = Module::from_binary(&engine, BOOTSTRAP_BINARY).unwrap();
    let main_instance = linker.instantiate(&mut store, &main).unwrap();
    let start = main_instance.get_typed_func::<(), (), _>(
      &mut store, "_start"
    ).unwrap();
    start.call(&mut store, ()).unwrap();
    0
  }
}

struct Run;

impl Display for Run {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "run".fmt(f)
  }
}

impl Command for Run {
  fn run(&self, cli: &mut Cli) -> u32 {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let owner = &cli.owners[cli.owner_selected.unwrap()];
    let model = &cli.models[cli.model_selected.unwrap()];
    let wasi_ctx = WasiCtxBuilder::new()
      .inherit_stdio()
      .args(&[
        "dropin_bootstrap.wasm".to_string(),
        format!("{}:{}:v1", owner, model),
      ]).unwrap()
      .preopened_dir(
        Dir::from_std_file(File::open(&cli.root).unwrap()),
        Path::new("/"),
      ).unwrap()
      .build();
    let mut store = Store::new(&engine, wasi_ctx);
    let std = Module::from_binary(&engine, STD_BINARY).unwrap();
    let std_instance = linker.instantiate(&mut store, &std).unwrap();
    linker.instance(
      &mut store, "blueforest:dropin-std:v1", std_instance,
    ).unwrap();
    let main = Module::from_file(&engine, get_build(cli)).unwrap();
    let main_instance = linker.instantiate(&mut store, &main).unwrap();
    let start = main_instance.get_typed_func::<(), (), _>(
      &mut store, "_start"
    ).unwrap();
    start.call(&mut store, ()).unwrap();
    0
  }
}
