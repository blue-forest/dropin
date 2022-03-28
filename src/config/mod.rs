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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use std::fmt::Display;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

mod error;
use error::ConfigError;
mod model;
use model::ModelCommand;
mod owner;
use owner::OwnerCommand;
mod root;
use root::get_root;
mod utils;

pub struct Cli {
  model_selected: Option<usize>,
  models:         Vec<String>,
  owner_selected: Option<usize>,
  owners:         Vec<String>,
  root:           PathBuf,
}

impl Cli {
  pub fn new() -> Self {
    let root = get_root();
    let owners = if !root.exists() {
      println!("Created drop'in root");
      create_dir(&root).unwrap();
      Vec::new()
    } else {
      get_dirs(&root)
    };
    Self{
      model_selected: None,
      models:         Vec::new(),
      owner_selected: None,
      owners,
      root,
    }
  }

  #[inline(always)]
  pub fn run(&mut self) {
    let commands: Vec<Box<dyn Command>> = vec![
      Box::new(OwnerCommand{}),
      Box::new(ModelCommand{}),
    ];
    self.run_select("Home", &commands);
  }

  fn run_select(&mut self, title: &str, commands: &[Box<dyn Command>]) {
    let theme = ColorfulTheme::default();
    loop {
      let enabled_commands: Vec<&Box<dyn Command>> = commands.iter()
        .filter(|x| x.is_enabled(self))
        .collect();
      let mut select = Select::with_theme(&theme);
      select.item("exit")
        .items(&enabled_commands)
        .default(1);
      select.with_prompt(self.prompt(title));
      let command = select.interact().unwrap();
      if command == 0 { break; }
      if commands[command-1].run(self) { break; }
    }
  }

  fn prompt(&self, title: &str) -> String {
    let mut result = String::new();
    result.push_str(
      if let Some(owner) = self.owner_selected {
        &self.owners[owner]
      } else {
        "<no owner selected>"
      }
    );
    result.push_str(": ");
    result.push_str(title);
    result
  }
}

impl Default for Cli {
  fn default() -> Self { Self::new() }
}

trait Command: Display {
  fn run(&self, cli: &mut Cli) -> bool;
  fn is_enabled(&self, _cli: &Cli) -> bool { true }
}

fn get_dirs(path: &Path) -> Vec<String> {
  let mut result = Vec::new();
  for entry in path.read_dir().unwrap() {
    if let Ok(owner_dir) = entry {
      let path = owner_dir.path();
      if path.is_dir() {
        result.push(
          path.file_name().unwrap().to_str().unwrap().to_string(),
        );
      }
    }
  }
  result
}
