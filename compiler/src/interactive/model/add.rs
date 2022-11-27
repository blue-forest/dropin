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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

use std::fmt::{Display, Error, Formatter};
use std::fs::create_dir_all;

use crate::interactive::path::get_owner;
use crate::interactive::{Cli, Command};
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
			let model_name: String =
				Input::with_theme(&ColorfulTheme::default())
					.with_prompt(
						"Model name for your recipes ? (leave empty to cancel)",
					)
					.allow_empty(true)
					.interact_text()
					.unwrap();
			if model_name.is_empty() {
				return 0;
			}
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
