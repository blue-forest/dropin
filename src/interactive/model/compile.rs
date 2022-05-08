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

use humantime::format_duration;

use std::fmt::{Display, Error, Formatter};
use std::time::{Duration, SystemTime};

use crate::interactive::{Cli, Command};

pub struct Compile;

impl Display for Compile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        "compile".fmt(f)
    }
}

impl Command for Compile {
    fn run(&self, cli: &mut Cli) -> u32 {
        let owner = &cli.owners[cli.owner_selected.unwrap()];
        let model = &cli.models[cli.model_selected.unwrap()];
        let start = SystemTime::now();
        cli.embedder.compile(&cli.root, owner, model);
        let elapsed = SystemTime::now().duration_since(start).unwrap();
        println!(
            "Compiled in {}",
            format_duration(Duration::new(
                elapsed.as_secs(),
                elapsed.as_millis() as u32 * 1_000_000
            ),)
        );
        0
    }
}
