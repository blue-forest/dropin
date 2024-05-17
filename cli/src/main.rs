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
use clap::{Parser, Subcommand, ValueEnum};
use dropin_compiler_recipes::ir::Component;
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
		path: PathBuf,
		#[arg(short, long, name = "compilation target")]
		target: Target,
	},
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Target {
	Dart,
	// Typescript,
	// Wasm,
	Debug,
}

fn main() -> Result<()> {
	let args = Args::parse();

	match args.command {
		Commands::Compile { path, target } => {
			let mut f = File::open(path)?;
			let mut recipe = String::new();
			f.read_to_string(&mut recipe)?;
			let ir = serde_yaml::from_str::<Component>(&recipe)?;
			let mut protobuf = vec![];
			ir.encode(&mut protobuf).unwrap();
			let protobuf = Box::into_raw(protobuf.into_boxed_slice());
			// println!("{ir:#?}");
			let output = match target {
				Target::Dart => todo!("dart"),
				Target::Debug => dropin_target_debug::codegen(protobuf),
			}
			.into_string()
			.unwrap();
			println!("{output}");
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
