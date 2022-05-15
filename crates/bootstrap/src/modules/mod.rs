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

use wasm_encoder::ValType::I32;
use wasm_encoder::{Instruction, Module, ValType};

use std::collections::HashMap;
use std::path::Path;

use dropin_core::print_to;

use crate::expressions::Expression;
use crate::path::get_recipe;
use crate::{Recipe, WasiUnwrap};

mod builder;
use builder::{Core, FunctionBuilder, Local, Locals, ModuleBuilder};

mod error;
pub use error::CompileError;

struct State<'a> {
	pub std_: Core<'a>,
}

#[derive(Default)]
struct FunctionState<'a> {
	stack: HashMap<&'a str, Local>,
	heap: HashMap<&'a str, (Local, Local)>,
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
		iter.next(); // skip recipes
		let mut function = iter.next().wasi_unwrap().iter();
		let mut function_state = FunctionState::default();

		let mut expression = function.next().wasi_unwrap();
		let mut is_public = false;
		if expression.pattern() == "public" {
			is_public = true;
			expression = function.next().wasi_unwrap();
		}

		let mut function_name = expression.as_str();
		if function_name == "main" {
			function_name = "_start";
		}

		expression = function.next().wasi_unwrap();
		let mut params = Params::default();
		if expression.pattern() == "params" {
			self.params(&mut function_state, &mut params, &expression);
			expression = function.next().wasi_unwrap();
		}

		let type_id = builder.type_(params.types, vec![]);
		let mut function = FunctionBuilder::new(
			type_id,
			if is_public { Some(function_name) } else { None },
			params.locals,
		);
		self.meta_commands(
			&mut builder,
			&mut state,
			&function_state,
			&mut function,
			&expression,
		);
		builder.function(function);

		Ok(builder.build())
	}

	fn params(
		&self,
		function_state: &mut FunctionState<'module>,
		params: &mut Params,
		expression: &Expression<'_, 'module>,
	) {
		for param in expression.iter() {
			let mut iter = param.iter();
			let key = iter.next().wasi_unwrap().as_str();
			let type_ = iter.next().wasi_unwrap().as_str();
			match type_ {
				"bytes" => {
					let base_id = params.push(ValType::I32);
					let len_id = params.push(ValType::I32);
					function_state
						.heap
						.insert(key, (Local::I32(base_id), Local::I32(len_id)));
				}
				_ => {
					print_to(&format!("unknown type: {}", type_), 2);
					unsafe { wasi::proc_exit(1) };
					unreachable!();
				}
			}
		}
	}

	fn meta_commands(
		&self,
		builder: &mut ModuleBuilder<'module>,
		state: &mut State<'module>,
		function_state: &'module FunctionState<'module>,
		function: &mut FunctionBuilder<'module>,
		expression: &Expression<'_, 'module>,
	) {
		for command in expression.iter() {
			let command_child = command.iter().next().wasi_unwrap();
			match command.pattern() {
				"metaCommand" => {
					self.meta_command(command_child);
				}
				"localCommand" => {
					self.local_command(
						builder,
						state,
						&function_state,
						function,
						command_child.iter().next().wasi_unwrap(),
					);
				}
				_ => {
					unreachable!()
				}
			}
		}
	}

	fn meta_command(&self, expression: &Expression) {
		match expression.pattern() {
			"print" => print_to(
				expression
					.iter()
					.next()
					.wasi_unwrap()
					.iter()
					.next()
					.wasi_unwrap()
					.iter()
					.next()
					.wasi_unwrap()
					.as_str(),
				2,
			),
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
		function_state: &'module FunctionState<'module>,
		function: &mut FunctionBuilder<'module>,
		expression: &Expression<'_, 'module>,
	) {
		match expression.pattern() {
			"print" => {
				let value = Value::from_expression(
					function_state, expression.iter().next().wasi_unwrap(),
				);
				let print = builder.get_core(&state.std_.print);
				value.print(builder, function);
				function.basic(Instruction::Call(print));
				/*
				let alloc = builder.get_core(&state.std_.alloc);
				let print = builder.get_core(&state.std_.print);
				let data = builder.memory().passive(message.as_bytes()) as u32;
				let ptr = function.add_local(I32);
				function.basic(Instruction::I32Const(message.len() as i32)); // size
				function.basic(Instruction::I32Const(1)); // align
				function.basic(Instruction::Call(alloc)); // -> ptr
				function.local(ptr.clone(), Instruction::LocalSet);
				function.local(ptr.clone(), Instruction::LocalGet);
				function.basic(Instruction::I32Const(0)); // offset
				function.basic(Instruction::I32Const(message.len() as i32)); // size
				function.basic(Instruction::MemoryInit { mem: 0, data });
				function.local(ptr, Instruction::LocalGet);
				function.basic(Instruction::I32Const(message.len() as i32)); // len
				function.basic(Instruction::Call(print));
				*/
			}
			pattern => {
				print_to(&format!("unknown command: {}", pattern), 2);
				unsafe { wasi::proc_exit(1) };
				unreachable!();
			}
		}
	}

	pub fn get_syntax(&self) -> String {
		let id = self.module.expression.iter().next().wasi_unwrap().as_str();
		get_recipe("syntaxes", id)
	}
}

#[derive(Default)]
struct Params {
	locals: Locals,
	types: Vec<ValType>,
}

impl Params {
	pub fn push(&mut self, type_: ValType) -> u32 {
		let result = self.types.len() as u32;
		self.types.push(type_);
		self.locals.add_local(type_);
		result
	}
}

#[derive(Debug)]
enum Value<'a> {
  Text(&'a str),
  StackLocal(&'a Local),
  HeapLocal(&'a Local, &'a Local),
}

impl<'a> Value<'a> {
	pub fn from_expression(
		function_state: &'a FunctionState<'a>, expression: &Expression<'_, 'a>,
	) -> Self {
		let value = expression.iter().next().wasi_unwrap();
		match value.pattern() {
			"text" => { todo!() }
			"getter" => {
				let query = value.iter().next().wasi_unwrap();
				let mut query_iter = query.iter();
				let top = query_iter.next().wasi_unwrap();
				match top.as_str() {
					"locals" => {
						let name = query_iter.next().wasi_unwrap().as_str();
						if let Some(local) = function_state.stack.get(name) {
							Self::StackLocal(local)
						} else if let Some((base, len)) = function_state.heap.get(name) {
							Self::HeapLocal(base, len)
						} else {
							print_to(&format!("local not found: {}", query.as_str()), 2);
							unsafe { wasi::proc_exit(1) };
							unreachable!();
						}
					}
					_ => {
						print_to(&format!("ref not found: {}", query.as_str()), 2);
						unsafe { wasi::proc_exit(1) };
						unreachable!();
					}
				}
			}
			_ => {
				print_to(&format!("unknown value: {}", value.pattern()), 2);
				unsafe { wasi::proc_exit(1) };
				unreachable!();
			}
		}
	}

	pub fn print(
		&self,
		_builder: &mut ModuleBuilder<'a>, 
		function: &mut FunctionBuilder<'a>,
	) {
		match self {
			Self::Text(_) => { todo!() }
			Self::StackLocal(_) => { todo!() }
			Self::HeapLocal(base, len) => {
				function.local((*base).clone(), Instruction::LocalGet);
				function.local((*len).clone(), Instruction::LocalGet);
			}
		}
	}
}
