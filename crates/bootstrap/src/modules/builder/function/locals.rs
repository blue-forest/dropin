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

use wasm_encoder::{Instruction, ValType};

use dropin_core::print_to;

use super::{FunctionBuilder, InstructionBuilder};

impl<'a> FunctionBuilder<'a> {
	pub fn local(&mut self, local: Local, cb: fn(u32) -> Instruction<'a>) {
		self.instructions
			.push_back(InstructionBuilder::Local(local, cb));
	}

	pub fn add_local(&mut self, type_: ValType) -> Local {
		self.locals.add_local(type_)
	}
}

#[derive(Default)]
pub struct Locals {
	pub i32_: u32,
	// pub i64_:       u32,
	// pub f32_:       u32,
	// pub f64_:       u32,
	// pub v128:       u32,
	// pub func_ref:   u32,
	// pub extern_ref: u32,
}

impl Locals {
	pub fn add_local(&mut self, type_: ValType) -> Local {
		match type_ {
			ValType::I32 => {
				let result = Local::I32(self.i32_);
				self.i32_ += 1;
				result
			}
			_ => {
				print_to(&format!("unknown type: {}", type_ as u32), 2);
				unsafe { wasi::proc_exit(1) };
				unreachable!();
			}
		}
	}

	pub fn resolve(local: &Local) -> u32 {
		/*
		if let &Local::I32(idx) = local {
		 idx
		} else {
		  unreachable!();
		}
		*/
		let &Local::I32(idx) = local;
		idx
	}

	pub fn build(&self) -> Vec<(u32, ValType)> {
		let mut result = vec![];
		if self.i32_ != 0 {
			result.push((self.i32_, ValType::I32));
		}
		result
	}
}

#[derive(Clone)]
pub enum Local {
	I32(u32),
}
