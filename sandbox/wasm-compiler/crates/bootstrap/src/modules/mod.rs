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

use wasm_encoder::ValType::I32;
use wasm_encoder::{Instruction, Module};

use std::path::Path;

use dropin_core::print_to;

use crate::expressions::Expression;
use crate::path::get_recipe;
use crate::{Recipe, WasiUnwrap};

mod builder;
use builder::{Core, ModuleBuilder};

mod error;
pub use error::CompileError;

struct State<'a> {
	pub std_: Core<'a>,
}

pub struct Compiler<'syntax, 'module> {
	module: Recipe<'syntax, 'module>,
}

impl<'syntax, 'module> Compiler<'syntax, 'module> {
	pub fn new(module: Recipe<'syntax, 'module>) -> Self {
		Self { module }
	}

	pub fn compile(&self, _path: &Path) -> Result<Module, CompileError> {
		let mut builder = ModuleBuilder::default();
		let mut state = State {
			std_: Core::default(),
		};

		let mut iter = self.module.expression.iter();
		iter.next(); // skip syntax
		let mut function = iter.next().wasi_unwrap().iter();
		let _function_name = function.next().wasi_unwrap().as_str();
		let commands = function.next().wasi_unwrap();

		for command in commands.iter() {
			let command_child = command.iter().next().wasi_unwrap();
			match command.pattern() {
				"metaCommand" => {
					self.meta_command(command_child);
				}
				"localCommand" => {
					self.local_command(
						&mut builder,
						&mut state,
						command_child.iter().next().wasi_unwrap(),
					);
				}
				_ => {
					unreachable!()
				}
			}
		}
		Ok(builder.build())
	}

	fn meta_command(&self, expression: &Expression) {
		match expression.pattern() {
			"print" => print_to(expression.iter().next().unwrap().as_str(), 2),
			pattern => {
				print_to(&format!("unknown command: {}", pattern), 2);
				unsafe { wasi::proc_exit(1) };
			}
		}
	}

	fn local_command(
		&self,
		builder: &mut ModuleBuilder<'module>,
		state: &mut State<'module>,
		expression: &Expression<'_, 'module>,
	) {
		match expression.pattern() {
			"print" => {
				let message = expression.iter().next().unwrap().as_str();
				let alloc = builder.get_core(&state.std_.alloc);
				let print = builder.get_core(&state.std_.print);
				let data = builder.memory().passive(message.as_bytes()) as u32;
				let start = builder.get_start();
				let ptr = start.add_local(I32);
				start.basic(Instruction::I32Const(message.len() as i32)); // size
				start.basic(Instruction::I32Const(1)); // align
				start.basic(Instruction::Call(alloc)); // -> ptr
				start.local(ptr.clone(), Instruction::LocalSet);
				start.local(ptr.clone(), Instruction::LocalGet);
				start.basic(Instruction::I32Const(0)); // offset
				start.basic(Instruction::I32Const(message.len() as i32)); // size
				start.basic(Instruction::MemoryInit { mem: 0, data });
				start.local(ptr, Instruction::LocalGet);
				start.basic(Instruction::I32Const(message.len() as i32)); // len
				start.basic(Instruction::Call(print));
			}
			pattern => {
				print_to(&format!("unknown command: {}", pattern), 2);
				unsafe { wasi::proc_exit(1) };
			}
		}
	}

	pub fn get_syntax(&self) -> String {
		let id = self.module.expression.iter().next().wasi_unwrap().as_str();
		get_recipe("syntaxes", id)
	}
}
