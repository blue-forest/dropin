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

use crate::{lexer::lexer, token::Token, Table};
use dropin_compiler_common::token::TokenKind;
use std::{
	fmt::{Debug, Formatter},
	vec,
};

pub struct Node<'a> {
	token: TokenKind<'a>,
	children: Vec<Node<'a>>,
	parent: Option<usize>,
	span: Option<(usize, usize)>,
}

impl<'a> Node<'a> {
	fn new(token: TokenKind<'a>, parent: Option<usize>) -> Node<'a> {
		Node {
			token,
			children: Vec::new(),
			parent,
			span: None,
		}
	}

	pub fn print(&self, input: &str, n_indent: usize) {
		let mut indent = String::new();
		for _ in 0..n_indent {
			indent.push_str("  ");
		}
		if let TokenKind::Eof | TokenKind::Empty = self.token {
			return;
		}
		println!("{}{}", indent, self.token.as_str());
		if let TokenKind::Terminal(_) = self.token {
			let span = self.span.unwrap();
			println!("{}\"{}\"", indent, &input[span.0..span.1],);
		}
	}
}

struct Stack<'a>(Vec<Node<'a>>);

impl Debug for Stack<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut first = true;
		for node in &self.0 {
			if !first {
				write!(f, ", ")?;
			}
			write!(f, "{:?}", node.token.as_str())?;
			first = false;
		}
		Ok(())
	}
}

impl<'a> Stack<'a> {
	fn new(main_non_terminal: Option<&'a str>) -> Stack<'a> {
		Stack(vec![
			Node::new(TokenKind::NonTerminal("root"), None),
			Node::new(
				TokenKind::NonTerminal(main_non_terminal.unwrap_or("predicate")),
				Some(0),
			),
		])
	}

	fn push(&mut self, node: Node<'a>) {
		self.0.push(node.into());
	}

	fn pop(&mut self) -> Node<'a> {
		self.0.pop().unwrap()
	}

	fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	fn into_tree(mut self) -> Node<'a> {
		self.0[0].children.pop().unwrap()
	}

	fn push_children(&mut self, node: Node<'a>) -> usize {
		let parent = node.parent.unwrap();
		if DEBUG {
			println!("PUSH_CHILDREN {:?} {:?}", node.token.as_str(), parent);
		}
		self.0[parent].children.push(node);
		parent
	}

	fn pop_children(&mut self, parent: usize) {
		self.0[parent].children.pop();
	}

	fn substitute(
		&mut self,
		mut parent: Option<usize>,
		substitute: &[TokenKind<'a>],
	) {
		while let Some(new_parent) = parent {
			println!("NEW_PARENT {:?}", new_parent);
			let TokenKind::NonTerminal(token) = self.0[new_parent].token else {
				unreachable!()
			};
			if token.ends_with("-") || self.0[new_parent].parent.is_none() {
				break;
			}
			parent = self.0[new_parent].parent;
		}
		if DEBUG {
			println!("SUBSTITUTE PARENT {:?}", parent);
		}
		self.0.extend(
			substitute
				.into_iter()
				.map(|token| Node::new(*token, parent)),
		);
	}
}

const DEBUG: bool = true;

pub fn parse<'a>(
	input: &str,
	main_non_terminal: Option<&'a str>,
	table: &'a Table,
) -> Node<'a> {
	if DEBUG {
		println!("{:?}", input);
	}

	let mut tokens = lexer(input);
	if DEBUG {
		println!("{:?}", tokens);
	}

	let mut stack = Stack::new(main_non_terminal);

	let mut is_deindent = false;
	let mut current = 0;

	while !stack.is_empty() {
		if DEBUG {
			println!("STACK {:?}", stack);
		}

		let stack_top = stack.pop();

		if let TokenKind::NonTerminal(token) = stack_top.token {
			let token_type = if current < tokens.len() {
				tokens[current].kind
			} else {
				TokenKind::Eof
			};

			let parent = if !token.ends_with("-") {
				Some(stack.push_children(stack_top))
			} else {
				None
			};

			let index = table.data.get(&token).unwrap().get(&token_type).unwrap();
			let substitute = &table.productions.get(*index);

			let substitute = if let Some(substitute) = substitute {
				substitute
			} else {
				if main_non_terminal.is_some() && token_type == TokenKind::Eof {
					break;
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
					continue;
				}
				panic!("{} unexpected {}", input, token);
			};

			if DEBUG {
				println!("STACK BEFORE SUBSTITUTION {:?}", stack);
				println!(
					"Substitution {} + {} => {}",
					token,
					token_type.as_str(),
					substitute
						.iter()
						.map(|sub| sub.as_str())
						.collect::<Vec<_>>()
						.join(", "),
				);
			}

			stack.substitute(parent, substitute);
		} else {
			todo!()
			/*
				NodeToken::Text(token) => {
					if token != "EMPTY" {
						if token != "EOF" {
							if DEBUG {
								println!("PUSH {}", token);
							}
							is_deindent = token == "DEINDENT";
							let parent = stack_top.parent.unwrap();
							stack_top.span = Some(tokens[current].span);
							parent.children.push(stack_top);
						}
						current += 1;
					}
				}
			}*/
		}

		if DEBUG {
			let now = std::time::Instant::now();
			while now.elapsed().as_millis() < 200 {
				// Do nothing
			}
		}
	}

	let root = stack.into_tree();

	if DEBUG {
		root.print(input, 0);
	}

	root
}
