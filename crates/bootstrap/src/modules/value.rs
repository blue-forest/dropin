use wasm_encoder::{Instruction, ValType::I32};

use crate::{Expression, print_to, WasiUnwrap};

use super::builder::{FunctionBuilder, Local, ModuleBuilder};
use super::functions::FunctionState;
use super::State;

#[derive(Debug)]
pub(in crate::modules) enum Value<'a> {
	Text(&'a str),
	StackLocal(&'a Local),
	HeapLocal(&'a Local, &'a Local),
}

impl<'a> Value<'a> {
	pub(in crate::modules) fn from_expression(
		function_state: &'a FunctionState<'a>,
		expression: &Expression<'_, 'a>,
	) -> Self {
		let value = expression.iter().next().wasi_unwrap();
		match value.pattern() {
			"text" => Self::Text(value.iter().next().wasi_unwrap().as_str()),
			"getter" => {
				let query = value.iter().next().wasi_unwrap();
				let mut query_iter = query.iter();
				let top = query_iter.next().wasi_unwrap();
				match top.as_str() {
					"locals" => {
						let name = query_iter.next().wasi_unwrap().as_str();
						if let Some(local) = function_state.stack.get(name) {
							Self::StackLocal(local)
						} else if let Some((base, len)) =
							function_state.heap.get(name)
						{
							Self::HeapLocal(base, len)
						} else {
							print_to(
								&format!("local not found: {}", query.as_str()),
								2,
							);
							unsafe { wasi::proc_exit(1) };
							unreachable!();
						}
					}
					_ => {
						print_to(
							&format!("ref not found: {}", query.as_str()),
							2,
						);
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

	pub(in crate::modules) fn print(
		&self,
		builder: &mut ModuleBuilder<'a>,
		state: &mut State<'a>,
		function: &mut FunctionBuilder<'a>,
	) {
		match self {
			Self::Text(message) => {
				let alloc = builder.get_core(&state.std.alloc);
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
			}
			Self::StackLocal(_) => {
				todo!()
			}
			Self::HeapLocal(base, len) => {
				function.local((*base).clone(), Instruction::LocalGet);
				function.local((*len).clone(), Instruction::LocalGet);
			}
		}
	}
}
