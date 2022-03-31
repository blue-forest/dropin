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

use std::path::PathBuf;

use crate::interactive::Cli;

fn push_owner(cli: &Cli, mut buf: PathBuf) -> Option<PathBuf> {
  if let Some(owner) = cli.owner_selected {
    buf.push(&cli.owners[owner]);
    Some(buf)
  } else {
    None
  }
}

#[allow(dead_code)] // TODO: use this instead of &cli.owners[owner]
pub fn get_owner(cli: &Cli) -> Option<PathBuf> {
  push_owner(cli, cli.root.clone())
}

fn push_model(cli: &Cli, mut buf: PathBuf) -> Option<PathBuf> {
  if let Some(model) = cli.model_selected {
    buf.push(&cli.models[model]);
    Some(buf)
  } else {
    None
  }
}

#[allow(dead_code)] // TODO: use this instead of &cli.models[model]
pub fn get_model(cli: &Cli) -> Option<PathBuf> {
  let mut buf = cli.root.clone();
  buf = push_owner(cli, buf).unwrap();
  push_model(cli, buf)
}

fn push_version(cli: &Cli, mut buf: PathBuf) -> Option<PathBuf> {
  if !cli.version.is_empty() {
    buf.push(&cli.version);
    Some(buf)
  } else {
    None
  }
}

pub fn get_version(cli: &Cli) -> Option<PathBuf> {
  let mut buf = cli.root.clone();
  buf = push_owner(cli, buf).unwrap();
  buf = push_model(cli, buf).unwrap();
  push_version(cli, buf)
}
