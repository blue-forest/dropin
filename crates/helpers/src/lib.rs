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

pub mod error;
use error::PortableExpect;
#[macro_use]
pub mod io;
pub mod fs;
pub mod header;
pub use header::Header;

pub fn decompose_version(id: &str) -> (&str, &str, &str) {
	let mut split = id.split(':');
	let owner = split.next().pexpect("expected owner");
	let model = split.next().pexpect("expected owner");
	let version = split.next().pexpect("expected owner");
	(owner, model, version)
}

pub fn decompose_recipe(id: &str) -> (&str, &str, &str, &str) {
	let mut split = id.split(':');
	let owner = split.next().pexpect("expected owner");
	let model = split.next().pexpect("expected owner");
	let version = split.next().pexpect("expected owner");
	let recipe = split.next().pexpect("expected owner");
	(owner, model, version, recipe)
}
