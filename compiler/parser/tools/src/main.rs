/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
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

use std::{fmt::Write, fs::File, io::Read, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};
use dropin_compiler_parser_lib::{parse, Table};

#[derive(Parser)]
struct Args {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Debug { path: PathBuf },
}

fn main() -> Result<()> {
  let args = Args::parse();

  match args.command {
    Commands::Debug { path } => {
      let mut f = File::open(path)?;
      let mut recipe = String::new();
      f.read_to_string(&mut recipe)?;
      let ir = parse(&mut Printer, recipe, None, &Table::default());
      println!("{ir:?}");
    }
  }
  Ok(())
}

pub struct Printer;

impl Write for Printer {
  fn write_str(&mut self, s: &str) -> std::fmt::Result {
    print!("{s}");
    Ok(())
  }
}
