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
use std::sync::Arc;

mod error;
use error::ConfigError;
mod model;
use model::ModelCommand;
mod owner;
use owner::OwnerCommand;
mod recipe;
use recipe::{RecipeCommand, Type};
mod path;
use path::get_root;
mod utils;
use utils::get_dirs;

pub struct Cli {
  model_selected: Option<usize>,
  models:         Vec<String>,
  owner_selected: Option<usize>,
  owners:         Vec<String>,
  root:           PathBuf,
  version:        String,
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
      version: "v1".to_string(), // TODO: deal with versions
    }
  }

  #[inline(always)]
  pub fn run(&mut self) {
    let commands: Vec<Box<dyn Command>> = vec![
      Box::new(RecipeCommand::new(Arc::new(Type))),
      Box::new(ModelCommand{}),
      Box::new(OwnerCommand{}),
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
      select.item("◀ back")
        .items(&enabled_commands)
        .default(1);
      select.with_prompt(self.prompt(title));
      let command = select.interact().unwrap();
      if command == 0 { break; }
      if enabled_commands[command-1].run(self) { break; }
    }
  }

  fn prompt(&self, title: &str) -> String {
    let mut result = String::new();
    if let Some(owner) = self.owner_selected {
      result.push_str(&self.owners[owner]);
      if let Some(model) = self.model_selected {
        result.push_str(":");
        result.push_str(&self.models[model]);
        result.push_str(":");
        result.push_str(&self.version);
      }
    } else {
      result.push_str("<no owner selected>");
    }
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

