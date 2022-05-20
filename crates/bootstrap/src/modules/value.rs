use wasm_encoder::{Instruction, ValType::I32};

use dropin_helpers::PortableUnwrap;

use crate::Expression;

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
		let value = expression.iter().next().punwrap();
		match value.pattern() {
			"text" => Self::Text(value.iter().next().punwrap().as_str()),
			"getter" => {
				let query = value.iter().next().punwrap();
				let mut query_iter = query.iter();
				let top = query_iter.next().punwrap();
				match top.as_str() {
					"locals" => {
						let name = query_iter.next().punwrap().as_str();
						if let Some(local) = function_state.stack.get(name) {
							Self::StackLocal(local)
						} else if let Some((base, len)) = function_state.heap.get(name) {
							Self::HeapLocal(base, len)
						} else {
							panic!("local not found: {}", query.as_str());
						}
					}
					_ => {
						panic!("ref not found: {}", query.as_str());
					}
				}
			}
			_ => {
				panic!("unknown value: {}", value.pattern());
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
