use std::collections::HashMap;
use std::slice::Iter;

use dropin_helpers::header::{Header, HeaderFunction};
use dropin_helpers::PortableUnwrap;

use crate::{print_to, Expression};

use super::builder::{FunctionBuilder, Local, ModuleBuilder};
use super::Compiler;

mod instructions;

mod params;
use params::Params;

#[derive(Default)]
pub struct FunctionState<'a> {
	pub(in crate::modules) stack: HashMap<&'a str, Local>,
	pub(in crate::modules) heap: HashMap<&'a str, (Local, Local)>,
}

impl<'syntax, 'module> Compiler<'syntax, 'module> {
	pub(in crate::modules) fn fn_profile<'internal: 'module>(
		&self,
		item: &mut Header<'internal>,
		builder: &mut ModuleBuilder<'module>,
		function_state: &mut FunctionState<'module>,
		function: &mut Iter<Expression<'_, 'internal>>,
	) -> FunctionBuilder<'module> {
		let mut expression = function.next().punwrap();
		let mut is_public = false;
		if expression.pattern() == "public" {
			is_public = true;
			expression = function.next().punwrap();
		}

		let mut function_name = expression.as_str();
		if function_name == "main" {
			function_name = "_start";
		}
		let mut item_function = HeaderFunction::new(function_name);

		let mut params = Params::default();
		expression = function.next().punwrap();
		if expression.pattern() == "params" {
			self.params(&mut item_function, function_state, &mut params, &expression);
		} else {
			print_to(expression.pattern(), 2);
		}

		item.push(item_function);

		let type_id = builder.type_(params.types, vec![]);
		FunctionBuilder::new(
			type_id,
			if is_public { Some(function_name) } else { None },
			params.locals,
		)
	}
}
