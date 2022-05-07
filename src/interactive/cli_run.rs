use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use std::sync::Arc;

use super::{Cli, Command};
use super::model::Models;
use super::owner::OwnerCommand;
use super::recipe::{Modules, RecipeCommand, Syntaxes};

impl Cli {
  #[inline(always)]
  pub fn run(&mut self) {
    self.run_select("Home", |_| vec![
      Box::new(RecipeCommand::new(Arc::new(Modules))),
      Box::new(RecipeCommand::new(Arc::new(Syntaxes))),
      Box::new(Models{}),
      Box::new(OwnerCommand{}),
    ]);
  }

  pub fn run_select<F: Fn(&mut Self) -> Vec<Box<dyn Command>>>(
    &mut self,
    title: &str,
    commands: F,
  ) -> u32 {
    let theme = ColorfulTheme::default();
    loop {
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
