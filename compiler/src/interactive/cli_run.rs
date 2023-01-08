/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2023 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use std::sync::Arc;

use super::model::Models;
use super::owner::OwnerCommand;
use super::recipe::{Modules, RecipeCommand, Syntaxes};
use super::{Cli, Command};

impl Cli {
	#[inline(always)]
	pub fn run(&mut self) {
		self.run_select("Home", |_| {
			vec![
				Box::new(RecipeCommand::new(Arc::new(Modules))),
				Box::new(RecipeCommand::new(Arc::new(Syntaxes))),
				Box::new(Models {}),
				Box::new(OwnerCommand {}),
			]
		});
	}

	pub fn run_select<F: Fn(&mut Self) -> Vec<Box<dyn Command>>>(
		&mut self,
		title: &str,
		commands: F,
	) -> u32 {
		let theme = ColorfulTheme::default();
		loop {
			let current_commands = commands(self);
			let enabled_commands: Vec<&Box<dyn Command>> = current_commands
				.iter()
				.filter(|x| x.is_enabled(self))
				.collect();
			let mut select = Select::with_theme(&theme);
			select.item("◀ back").items(&enabled_commands).default(1);
			select.with_prompt(self.prompt(title));
			let command = select.interact().unwrap();
			if command == 0 {
				break 0;
			}
			let back_n = enabled_commands[command - 1].run(self);
			if back_n > 0 {
				break back_n - 1;
			}
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
