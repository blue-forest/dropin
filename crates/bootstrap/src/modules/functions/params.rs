use wasm_encoder::ValType;

use dropin_helpers::header::{HeaderFunction, HeaderParam, HeaderType};
use dropin_helpers::PortableUnwrap;

use crate::modules::builder::{Local, Locals};
use crate::modules::Compiler;
use crate::Expression;

use super::FunctionState;

impl<'syntax, 'module> Compiler<'syntax, 'module> {
	pub(in crate::modules) fn params<'internal: 'module>(
		&self,
		header: &mut HeaderFunction<'internal>,
		function_state: &mut FunctionState<'module>,
		params: &mut Params,
		expression: &Expression<'_, 'internal>,
	) {
		for param in expression.iter() {
			let mut iter = param.iter();
			let key = iter.next().punwrap().as_str();
			let type_ = iter.next().punwrap().as_str();
			let header_type = match type_ {
				"bytes" => {
					let base_id = params.push(ValType::I32);
					let len_id = params.push(ValType::I32);
					function_state
						.heap
						.insert(key, (Local::I32(base_id), Local::I32(len_id)));
					HeaderType::Bytes
				}
				_ => {
					panic!("unknown type: {}", type_);
				}
			};
			header.push(HeaderParam::new(key, header_type));
		}
	}
}

#[derive(Default)]
pub struct Params {
	pub(super) locals: Locals,
	pub(super) types: Vec<ValType>,
}

impl Params {
	pub fn push(&mut self, type_: ValType) -> u32 {
		let result = self.types.len() as u32;
		self.types.push(type_);
		self.locals.add_local(type_);
		result
	}
}
