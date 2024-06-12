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

use std::{fmt::Write, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use dropin_compiler_recipes::parser::parse_model;
use prost::Message;

#[derive(Parser)]
#[command(
	about = env!("CARGO_PKG_DESCRIPTION"),
	version = env!("CARGO_PKG_VERSION"),
	long_about = None
)]
struct Args {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Compile {
		#[arg(name = "compilation target")]
		target: Target,
		path: PathBuf,
	},
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Target {
	Flutter,
	// Typescript,
	// Wasm,
}

fn main() -> Result<()> {
	let args = Args::parse();

	match args.command {
		Commands::Compile { path, target } => {
			let ir = parse_model(&path).unwrap();
			println!("{ir:#?}");
			let mut protobuf = vec![];
			ir.encode(&mut protobuf).unwrap();
			let protobuf = Box::into_raw(protobuf.into_boxed_slice());
			let output = match target {
				Target::Flutter => dropin_target_flutter::codegen(protobuf),
			};
			println!("{output:?}");
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
