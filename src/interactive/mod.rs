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

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use std::fmt::Display;
use std::fs::create_dir;
use std::path::PathBuf;
use std::sync::Arc;

use dropin_utils::path::get_root;

mod config;
use self::config::Config;
mod error;
use error::ConfigError;
mod model;
use model::ModelCommand;
mod owner;
use owner::OwnerCommand;
mod recipe;
use recipe::{Modules, RecipeCommand, Syntaxes};
mod path;
use path::validate_path;
mod utils;
use utils::get_dirs;

pub struct Cli {
  config:         Config,
  cwd:            PathBuf,
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
    validate_path(&root).unwrap();
    let owners = if !root.exists() {
      println!("Created drop'in root");
      create_dir(&root).unwrap();
      vec![]
    } else {
      get_dirs(&root)
    };
    let mut cwd = root.clone();
    let config = Config::new(&root);
    let mut owner_selected = None;
    let mut model_selected = None;
    let mut models = vec![];
    if let Some(owner) = config.owner() {
      owner_selected = Some(owners.iter().position(|o| o == owner).unwrap());
      cwd.push(owner);
      cwd.push("models");
      models = get_dirs(&cwd);
      if let Some(model) = config.model() {
        cwd.push(model);
        cwd.push("v1"); // TODO: deal with versions
        model_selected = Some(models.iter().position(|m| m == model).unwrap());
      }
    }
    Self{
      config,
      cwd,
      model_selected,
      models,
      owner_selected,
      owners,
      root,
      version: "v1".to_string(), // TODO: deal with versions
    }
  }

  #[inline(always)]
  pub fn run(&mut self) {
    self.run_select("Home", |_| vec![
      Box::new(RecipeCommand::new(Arc::new(Modules))),
      Box::new(RecipeCommand::new(Arc::new(Syntaxes))),
      Box::new(ModelCommand{}),
      Box::new(OwnerCommand{}),
    ]);
  }

  fn run_select<F: Fn(&mut Self) -> Vec<Box<dyn Command>>>(
    &mut self,
    title: &str,
    commands: F,
  ) -> u32 {
    let theme = ColorfulTheme::default();
    loop {
      println!("CWD: {}", self.cwd.to_str().unwrap());
      let current_commands = commands(self);
      let enabled_commands: Vec<&Box<dyn Command>> = current_commands.iter()
        .filter(|x| x.is_enabled(self))
        .collect();
      let mut select = Select::with_theme(&theme);
      select.item("◀ back")
        .items(&enabled_commands)
        .default(1);
      select.with_prompt(self.prompt(title));
      let command = select.interact().unwrap();
      if command == 0 { break 0; }
      let back_n = enabled_commands[command-1].run(self);
      if back_n > 0 { break back_n - 1; }
    }
  }

  fn prompt(&self, title: &str) -> String {
    let mut result = String::new();
    if let Some(owner) = self.owner_selected {
      result.push_str(&self.owners[owner]);
      if let Some(model) = self.model_selected {
        result.push(':');
        result.push_str(&self.models[model]);
        result.push(':');
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
  fn run(&self, cli: &mut Cli) -> u32;
  fn is_enabled(&self, _cli: &Cli) -> bool { true }
}

