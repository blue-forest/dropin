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
use std::path::PathBuf;

mod error;
use error::ConfigError;
mod root;
use root::get_root;
mod owner;
use owner::OwnerCommand;

pub struct Cli {
  root:           PathBuf,
  owners:         Vec<String>,
  selected_owner: Option<usize>,
}

impl Cli {
  pub fn new() -> Self {
    let root = get_root();
    let mut owners = Vec::new();
    if !root.exists() {
      println!("Created drop'in root");
      create_dir(&root).unwrap();
    } else {
      for entry in root.read_dir().unwrap() {
        if let Ok(owner_dir) = entry {
          let path = owner_dir.path();
          if path.is_dir() {
            owners.push(
              path.file_name().unwrap().to_str().unwrap().to_string(),
            );
          }
        }
      }
    }
    Self{ root, owners, selected_owner: None }
  }

  pub fn run(&mut self) {
    let commands: Vec<Box<dyn Command>> = vec![ Box::new(OwnerCommand{}) ];
    self.run_select("Home", &commands);
  }

  fn run_select(&mut self, title: &str, commands: &[Box<dyn Command>]) {
    let theme = ColorfulTheme::default();
    let mut select = Select::with_theme(&theme);
    select.item("exit")
      .items(&commands)
      .default(1);
    loop {
      select.with_prompt(self.prompt(title));
      let command = select.interact().unwrap();
      if command == 0 { break; }
      if commands[command-1].run(self) { break; }
    }
  }

  fn prompt(&self, title: &str) -> String {
    let mut result = String::new();
    result.push_str(
      if let Some(owner) = self.selected_owner {
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
}
