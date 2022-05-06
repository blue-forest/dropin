use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;

use std::fmt::{Display, Error, Formatter};
use std::fs::create_dir_all;

use crate::interactive::{Cli, Command};
use crate::interactive::path::get_owner;
use crate::utils::validate_name;

pub struct Add;

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
