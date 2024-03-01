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

struct Node {
	token: NodeToken,
	children: Vec<Node>,
	parent: Option<Box<Node>>,
	span: Option<(usize, usize)>,
}

impl Node {
	fn new(token: NodeToken, parent: Option<Box<Node>>) -> Node {
		Node {
			token,
			children: Vec::new(),
			parent,
			span: None,
		}
	}
}

#[derive(PartialEq)]
enum NodeToken {
	Text(String),
	Quantity(u64),
}

const DEBUG: bool = false;

fn parse(input: &str, main_non_terminal: Option<&str>, table: &Table) -> Node {
	if DEBUG {
		println!("{:?}", input);
	}

	let tokens = lexer(input);
	if DEBUG {
		println!("{:?}", tokens);
	}

	let mut root = Node::new(NodeToken::Text("EOF".to_string()), None);

	let mut main_non_terminal_id = 0;
	if let Some(main_non_terminal) = main_non_terminal {
		for (id, non_terminal) in table.non_terminals.iter() {
			if non_terminal == &main_non_terminal {
				main_non_terminal_id = *id;
				break;
			}
		}
	}

	let mut stack = vec![
		root,
		Node::new(
			NodeToken::Quantity(main_non_terminal_id),
			Some(Box::new(root)),
		),
	];

	let mut is_deindent = false;
	let mut current = 0;

	while !stack.is_empty() {
		if DEBUG {
			println!(
				"STACK {:?}",
				stack
					.iter()
					.map(|node| {
						match &node.token {
							NodeToken::Quantity(token) => {
								*table.non_terminals.get(token).unwrap()
							}
							NodeToken::Text(token) => token,
						}
					})
					.collect::<Vec<&str>>()
			);
		}

		let stack_top = stack.pop().unwrap();
		match stack_top.token {
			NodeToken::Quantity(token) => {
				let token_type = if current < tokens.len() {
					tokens[current].kind
				} else {
					TokenKind::Eof
				};

				if !table.non_terminals.get(&token).unwrap().ends_with("-") {
					let parent = stack_top.parent.unwrap();
					parent.children.push(stack_top);
				}

				let index = table.data[&token][&token_type];
				let substitute = table.productions[index].clone();

				if substitute.is_none() {
					if main_non_terminal.is_some() && token_type == TokenKind::Eof {
						break;
					}
					if is_deindent {
						if DEBUG {
							println!("NEWLINE after DEINDENT");
						}
						tokens.insert(current, Token::new(TokenKind::Newline, (0, 0)));
						stack.push(stack_top);
						if !table.non_terminals.get(&token).unwrap().ends_with("-") {
							stack_top.parent.unwrap().children.pop();
						}
						is_deindent = false;
						continue;
					}
					panic!(
						"{} unexpected {}",
						input,
						table.non_terminals.get(&token).unwrap()
					);
				}

				if DEBUG {
					println!(
						"Substitution {} + {} => {}",
						table.non_terminals.get(&token).unwrap(),
						token_type.as_str(),
						substitute
							.iter()
							.map(|sub| {
								match sub {
									NodeToken::Quantity(token) => {
										*table.non_terminals.get(&token).unwrap()
									}
									NodeToken::Text(token) => &token,
								}
							})
							.collect::<Vec<&str>>(),
					);
				}

				let mut substitute_parent = stack_top;
				let NodeToken::Quantity(mut substitute_parent_token) =
					substitute_parent.token
				else {
					unreachable!()
				};
				while substitute_parent.token != NodeToken::Text("EOF".to_string())
					&& table
						.non_terminals
						.get(&substitute_parent_token)
						.unwrap()
						.ends_with("-")
				{
					substitute_parent = *substitute_parent.parent.unwrap();
					let NodeToken::Quantity(new_value) = substitute_parent.token else {
						unreachable!()
					};
					substitute_parent_token = new_value;
				}

				stack.extend(substitute.unwrap().iter().map(|token| {
					Node::new(token.clone(), Some(Box::new(substitute_parent)))
				}));
			}
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
		}

		if DEBUG {
			let now = std::time::Instant::now();
			while now.elapsed().as_millis() < 200 {
				// Do nothing
			}
		}
	}

	if DEBUG {
		print(input, &root.children[0], 0, table);
	}

	root.children[0]
}

fn print(input: &str, node: &Node, n_indent: usize, table: &Table) {
	let mut indent = String::new();
	for _ in 0..n_indent {
		indent.push_str("  ");
	}
	match &node.token {
		NodeToken::Quantity(token) => {
			if node.children.is_empty() {
				return;
			}
			println!("{}{}", indent, table.non_terminals.get(token).unwrap());
			for child in &node.children {
				print(input, child, n_indent + 1, table);
			}
		}
		NodeToken::Text(token) => {
			if token != "EMPTY" && token != "EOF" {
				let span = node.span.unwrap();
				println!(
					"{}{} \"{}\"",
					indent,
					token,
					input.chars().collect::<Vec<char>>()[span.0..span.1]
						.iter()
						.collect::<String>(),
				);
			}
		}
	}
}
