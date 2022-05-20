use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

use dropin_helpers::PortableUnwrap;

use super::skip_whitespaces;

#[derive(Default, Debug)]
pub struct Config<'syntax, 'module> {
	stacks: HashMap<&'syntax str, Vec<&'module str>>,
}

impl<'syntax, 'module> Config<'syntax, 'module> {
	pub(super) fn new(
		syntax: &'syntax str,
		iter: &mut Peekable<CharIndices<'syntax>>,
	) -> Self {
		let mut result = Self::default();
		let mut offset = skip_whitespaces(iter).unwrap_or(syntax.len());
		if !syntax.get(offset..).punwrap().starts_with("@config\n") {
			return result;
		}
		for _ in 0..8 {
			// skip "@config\n"
			iter.next().punwrap();
		}

		let mut stack_indent = vec![];
		offset += 8;
		let indent_end = loop {
			let peeked = iter.peek();
			if peeked.is_none() {
				break syntax.len();
			}
			let (i, c) = iter.next().punwrap();
			if !c.is_whitespace() {
				break i;
			}
		};
		stack_indent.push(syntax.get(offset..indent_end).punwrap());

		offset = indent_end;
		#[allow(clippy::while_let_on_iterator)] // if `for` is used, iter is moved
		while let Some((_, c)) = iter.next() {
			offset += 1;
			if c.is_whitespace() {
				break;
			}
		}
		let key = syntax.get(indent_end..offset).punwrap();

		match key {
			"stack" => result.stack(syntax, iter),
			_ => {
				panic!("unknown config key: {}", key);
			}
		}
		result
	}

	fn stack(
		&mut self,
		syntax: &'syntax str,
		iter: &mut Peekable<CharIndices<'syntax>>,
	) {
		let debug: String = iter.map(|(_, c)| c).collect();
		println!("{}", debug);
		todo!();
	}

	pub fn get(&self, key: &str) -> Option<&Vec<&'module str>> {
		self.stacks.get(key)
	}

	pub fn get_mut(&mut self, key: &str) -> Option<&mut Vec<&'module str>> {
		self.stacks.get_mut(key)
	}
}
