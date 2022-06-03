use wasm_encoder::Instruction;

use crate::modules::builder::{FunctionBuilder, ModuleBuilder};
use crate::modules::{Compiler, State, Value};
use crate::{print_to, Expression, WasiUnwrap};

use super::FunctionState;

impl<'syntax, 'module> Compiler<'syntax, 'module> {
	pub(in crate::modules) fn instructions(
		&self,
		builder: &mut ModuleBuilder<'module>,
		state: &mut State<'module>,
		function_state: &'module FunctionState<'module>,
		function: &mut FunctionBuilder<'module>,
		expression: &Expression<'_, 'module>,
	) {
		for instruction in expression.iter() {
			let instruction_child = instruction.iter().next().wasi_unwrap();
			match instruction.pattern() {
				"metaCommand" => {
					self.meta_instruction(instruction_child);
				}
				"localCommand" => {
					self.local_instruction(
						builder,
						state,
						&function_state,
						function,
						instruction_child.iter().next().wasi_unwrap(),
					);
				}
				_ => {
					unreachable!()
				}
			}
		}
	}

	pub(in crate::modules) fn meta_instruction(&self, expression: &Expression) {
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
				print_to(&format!("unknown instruction: {}", pattern), 2);
				unsafe { wasi::proc_exit(1) };
			}
		}
	}

	pub(in crate::modules) fn local_instruction(
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
					function_state,
					expression.iter().next().wasi_unwrap(),
				);
				let print = builder.get_core(&state.std.print);
				value.print(builder, state, function);
				function.basic(Instruction::Call(print));
			}
			pattern => {
				print_to(&format!("unknown instruction: {}", pattern), 2);
				unsafe { wasi::proc_exit(1) };
				unreachable!();
			}
		}
	}
}
