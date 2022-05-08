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

use std::fmt::{Display, Error, Formatter};

use crate::interactive::{Cli, Command};

pub struct Select {
    pub name: String,
    pub index: usize,
}

impl Display for Select {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.name.fmt(f)
    }
}

impl Command for Select {
    fn run(&self, cli: &mut Cli) -> u32 {
        cli.owner_selected = Some(self.index);
        cli.model_selected = None;
        cli.cwd = cli.root.clone();
        cli.cwd.push(&cli.owners[self.index]);
        cli.cwd.push("models");
        cli.config.set_owner(self.name.clone());
        1
    }
}
