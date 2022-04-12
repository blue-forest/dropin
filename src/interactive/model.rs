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

use std::fmt::{Display, Formatter, Error};
use std::fs::create_dir;

use super::{Cli, Command, get_dirs};
use super::utils::validate_name;

pub struct ModelCommand;

impl Display for ModelCommand {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    "models".fmt(f)
  }
}

impl Command for ModelCommand {
  fn run(&self, cli: &mut Cli) -> u32 {
    let owner_path = &cli.root.join(
      &cli.owners[cli.owner_selected.unwrap()]
    );
    cli.models = get_dirs(owner_path);
    let mut commands: Vec<Box<dyn Command>> = Vec::new();
    for (i, model) in cli.models.iter().enumerate() {
      commands.push(Box::new(Select{
        name:  model.to_string(),
        index: i,
      }));
    }
    commands.push(Box::new(Add{}));
    cli.run_select("Model", &commands)
  }

  fn is_enabled(&self, cli: &Cli) -> bool { cli.owner_selected.is_some() }
}

struct Select {
  name:  String,
  index: usize,
}

impl Display for Select {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    self.name.fmt(f)
  }
}

impl Command for Select {
  fn run(&self, cli: &mut Cli) -> u32 {
    cli.model_selected = Some(self.index);
    cli.config.set_model(self.name.clone());
    1
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
    let owner_path = &cli.root.join(
      &cli.owners[cli.owner_selected.unwrap()]
    );
    let (model_name, model_path) = loop {
      let model_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Model name for your recipes ? (leave empty to cancel)")
        .allow_empty(true)
        .interact_text().unwrap();
      if model_name.is_empty() { return 0; }
      let model_path = owner_path.join(&model_name);
      if let Err(err) = validate_name(&model_path, &model_name) {
        println!("{}", err);
        continue;
      }
      break (model_name, model_path);
    };
    create_dir(&model_path).unwrap();
    create_dir(&model_path.join("v1")).unwrap();
    println!("Model {} created", model_name);
    let index = cli.models.len();
    cli.models.push(model_name);
    cli.model_selected = Some(index);
    cli.config.set_model(cli.models[index].clone());
    1
  }
}

