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

use std::{
	fmt::Write,
	fs::{create_dir, remove_dir_all, File},
	io::Write as IoWrite,
	path::PathBuf,
};

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
		#[arg(long, short)]
		output: Option<PathBuf>,
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
		Commands::Compile {
			path,
			target,
			output,
		} => {
			let ir = parse_model(&path)?;
			let mut protobuf = vec![];
			ir.encode(&mut protobuf)?;
			let protobuf = Box::into_raw(protobuf.into_boxed_slice());
			let code = match target {
				Target::Flutter => dropin_target_flutter::codegen(protobuf),
			};
			let code = unsafe { Box::from_raw(code) };
			if let Some(output) = output {
				if output.exists() {
					remove_dir_all(&output)?;
				}
				create_dir(&output)?;
				for (path, content) in *code {
					let path = path
						.split('/')
						.fold(output.clone(), |path, key| path.join(key));
					let mut file = File::create(path)?;
					file.write(content.as_bytes())?;
				}
			} else {
				println!("{}", serde_json::to_string(&code)?);
			}
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
