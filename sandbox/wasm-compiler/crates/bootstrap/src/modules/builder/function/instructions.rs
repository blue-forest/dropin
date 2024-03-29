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

use wasm_encoder::Instruction;

use super::{Local, Locals};

pub enum InstructionBuilder<'a> {
	Basic(Instruction<'a>),
	Local(Local, fn(u32) -> Instruction<'a>),
}

impl<'a> InstructionBuilder<'a> {
	pub fn build(self) -> Instruction<'a> {
		match self {
			Self::Basic(result) => result,
			Self::Local(idx, cb) => cb(Locals::resolve(&idx)),
		}
	}
}
