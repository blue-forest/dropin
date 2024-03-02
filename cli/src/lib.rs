/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
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

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
	about = env!("CARGO_PKG_DESCRIPTION"),
	version = env!("CARGO_PKG_VERSION"),
	long_about = None
)]
pub struct Args {
	#[arg(short, long, name = "compilation target")]
	pub target: Target,
	#[arg(name = "drop'in code")]
	pub input: String,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Target {
	Dart,
	Typescript,
	Wasm,
}
