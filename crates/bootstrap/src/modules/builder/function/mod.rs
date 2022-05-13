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

use wasm_encoder::{Function, Instruction};

use std::collections::VecDeque;

use crate::sys::WasiUnwrap;

use super::ModuleBuilder;

mod instructions;
pub use instructions::InstructionBuilder;

mod locals;
pub use locals::{Local, Locals};

impl<'module> ModuleBuilder<'module> {
	pub fn get_start(&mut self) -> &mut FunctionBuilder<'module> {
		self.functions_local.get_mut(0).wasi_unwrap()
	}

	pub fn add_function(&mut self, func: FunctionBuilder<'module>) {
		self.functions_local.push_back(func);
	}
}

pub struct FunctionBuilder<'a> {
	type_id: u32,
	instructions: VecDeque<InstructionBuilder<'a>>,
	locals: Locals,
}

impl<'a> FunctionBuilder<'a> {
	pub fn new(type_id: u32) -> Self {
		Self {
			type_id,
			instructions: VecDeque::new(),
			locals: Locals::default(),
		}
	}

	pub fn type_id(&self) -> u32 {
		self.type_id
	}

	pub fn basic(&mut self, instruction: Instruction<'a>) {
		self.instructions
			.push_back(InstructionBuilder::Basic(instruction));
	}

	pub fn build(mut self) -> Function {
		let mut result = Function::new(self.locals.build());
		while let Some(i) = self.instructions.pop_front() {
			result.instruction(&i.build());
		}
		result.instruction(&Instruction::End);
		result
	}
}
