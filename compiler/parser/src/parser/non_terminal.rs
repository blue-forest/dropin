use dropin_compiler_common::token::TokenKind;

use crate::{token::Token, Table};

use super::{node::NodeBuilder, stack::Stack, LoopControl, DEBUG};

pub(super) fn parse_non_terminal(
	table: &Table,
	input: &str,
	tokens: &mut Vec<Token>,
	current: usize,
	stack: &mut Stack,
	stack_top_index: usize,
	name: &str,
	is_deindent: bool,
) -> LoopControl {
	let token_type = if current < tokens.len() {
		tokens[current].kind
	} else {
		TokenKind::Eof
	};

	let parent = if !name.ends_with("-") {
		stack.push_children(stack_top_index);
		Some(stack_top_index)
	} else {
		None
	};

	let index = table.data.get(&name).unwrap().get(&token_type).unwrap();
	let substitute = &table.productions.get(*index);

	let substitute = if let Some(substitute) = substitute {
		substitute
	} else {
		if token_type == TokenKind::Eof {
			return LoopControl::Break;
		}

		if is_deindent {
			if DEBUG {
				println!("NEWLINE after DEINDENT");
			}
			tokens.insert(current, Token::new(TokenKind::Newline, (0, 0)));
			if let Some(parent) = parent {
				stack.pop_children(parent);
			}
			is_deindent = false;
			return LoopControl::Continue;
		}
		panic!("{} unexpected {}", input, name);
	};

	if DEBUG {
		println!(
			"Substitution {} + {} => {}",
			name,
			token_type.as_str(),
			substitute
				.iter()
				.map(|sub| sub.as_str())
				.collect::<Vec<_>>()
				.join(", "),
		);
	}

	stack.substitute(Some(stack_top_index), substitute);

	LoopControl::Continue
}
