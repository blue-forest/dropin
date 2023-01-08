/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2023 Blue Forest
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

use std::fmt::{Display, Error, Formatter};

use super::{Cli, Command};

mod add;
use add::Add;
mod select;
use select::Select;

pub struct OwnerCommand;

impl Display for OwnerCommand {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		"owners".fmt(f)
	}
}

impl Command for OwnerCommand {
	fn run(&self, cli: &mut Cli) -> u32 {
		cli.run_select("Owner", |cli| {
			let mut commands: Vec<Box<dyn Command>> = Vec::new();
			for (i, owner) in cli.owners.iter().enumerate() {
				commands.push(Box::new(Select {
					name: owner.to_string(),
					index: i,
				}));
			}
			commands.push(Box::new(Add {}));
			commands
		})
	}
}
