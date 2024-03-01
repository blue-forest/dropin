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

use dropin_compiler_common::token::TokenKind;

use crate::token::{Token, TokenState};

struct Tokens<'a>(Vec<Token<'a>>);

impl<'a> Tokens<'a> {
	pub fn push(&mut self, token: Token<'a>) {
		self.0.push(token);
	}

	pub fn next(
		&mut self,
		token: Option<Token<'a>>,
		end: usize,
	) -> Option<Token<'a>> {
		if let Some(mut current) = token {
			current.state = None;
			current.span.1 = end;
			self.0.push(current);
		}
		None
	}

	pub fn into_inner(self) -> Vec<Token<'a>> {
		self.0
	}
}

pub fn lexer(input: &str) -> Vec<Token> {
	let mut tokens = Tokens(Vec::new());
	let mut current: Option<Token> = None;
	let mut indent: Vec<usize> = vec![0];

	let mut index = 0;
	let bytes_length = input.len();
	let bytes = input.as_bytes();

	while index < bytes_length {
		let char = bytes[index];
		if current.is_none() {
			if char == b'\n' {
				let mut line_indent = 0;
				index += 1;
				let start = index;
				while index < bytes_length {
					let char = bytes[index];
					if char == b' ' {
						line_indent += 1;
					} else if char == b'\t' {
						line_indent += 2;
					} else {
						if indent[indent.len() - 1] < line_indent {
							indent.push(line_indent);
							tokens.push(Token::new(TokenKind::Indent, (start, index)));
						} else if indent[indent.len() - 1] > line_indent {
							indent.pop();
							tokens.push(Token::new(TokenKind::Deindent, (start, index)));
						} else {
							tokens.push(Token::new(TokenKind::Newline, (start, index)));
						}
						index -= 1;
						break;
					}
					index += 1;
				}
			} else if !is_whitespace(char) {
				if char.is_ascii_alphabetic() {
					if input[index..].starts_with("in") && is_whitespace(bytes[index + 2])
					{
						tokens.push(Token::new(TokenKind::In, (index, index + 2)));
						index += 1;
					} else if input[index..].starts_with("if")
						&& is_whitespace(bytes[index + 2])
					{
						tokens.push(Token::new(TokenKind::If, (index, index + 2)));
						index += 1;
					} else if input[index..].starts_with("else")
						&& !bytes[index + 4].is_ascii_alphabetic()
					{
						tokens.push(Token::new(TokenKind::Else, (index, index + 4)));
						index += 3;
					} else if input[index..].starts_with("false")
						&& (index + 5 >= bytes_length
							|| !bytes[index + 5].is_ascii_alphabetic())
					{
						tokens.push(Token::new(TokenKind::False, (index, index + 5)));
						index += 4;
					} else if input[index..].starts_with("true")
						&& (index + 4 >= bytes_length
							|| !bytes[index + 4].is_ascii_alphabetic())
					{
						tokens.push(Token::new(TokenKind::True, (index, index + 4)));
						index += 3;
					} else {
						current = Some(Token::new(TokenKind::Id, (index, index)));
					}
				} else if char == b'"' {
					current = Some(Token::new_with_state(
						TokenKind::Text,
						(index, index),
						Some(TokenState::new(Some(false), None)),
					));
					index += 1;
				} else if char.is_ascii_digit() {
					current = Some(Token::new_with_state(
						TokenKind::Quantity,
						(index, index),
						Some(TokenState::new(None, Some(false))),
					));
				} else if char == b'+' {
					tokens.push(Token::new(TokenKind::Add, (index, index + 1)));
				} else if char == b'-' {
					tokens.push(Token::new(TokenKind::Sub, (index, index + 1)));
				} else if char == b':' {
					tokens.push(Token::new(TokenKind::Block, (index, index + 1)));
				} else if char == b'.' {
					tokens.push(Token::new(TokenKind::Dot, (index, index + 1)));
				} else if char == b',' {
					tokens.push(Token::new(TokenKind::Comma, (index, index + 1)));
				} else if input[index..].starts_with(">=") {
					tokens.push(Token::new(TokenKind::AtLeast, (index, index + 2)));
					index += 1;
				} else if input[index..].starts_with("<=") {
					tokens.push(Token::new(TokenKind::AtMost, (index, index + 2)));
					index += 1;
				} else if input[index..].starts_with('>') {
					tokens.push(Token::new(TokenKind::MoreThan, (index, index + 1)));
				} else if input[index..].starts_with('<') {
					tokens.push(Token::new(TokenKind::LessThan, (index, index + 1)));
				} else if input[index..].starts_with("==") {
					tokens.push(Token::new(TokenKind::EqualsTo, (index, index + 2)));
					index += 1;
				} else if input[index..].starts_with("!=") {
					tokens.push(Token::new(TokenKind::DifferentFrom, (index, index + 2)));
					index += 1;
				} else if input[index..].starts_with('!') {
					tokens.push(Token::new(TokenKind::Not, (index, index + 1)));
				} else if input[index..].starts_with('&') {
					tokens.push(Token::new(TokenKind::And, (index, index + 1)));
				} else if input[index..].starts_with('|') {
					tokens.push(Token::new(TokenKind::Or, (index, index + 1)));
				} else if input[index..].starts_with('=') {
					tokens.push(Token::new(TokenKind::Samekey, (index, index + 1)));
				} else if input[index..].starts_with('(') {
					tokens.push(Token::new(
						if is_spaced(bytes, index) {
							TokenKind::ParSpaced
						} else {
							TokenKind::ParGlued
						},
						(index, index + 1),
					));
				} else if input[index..].starts_with(')') {
					tokens.push(Token::new(TokenKind::Rpar, (index, index + 1)));
				} else if input[index..].starts_with('[') {
					tokens.push(Token::new(
						if is_spaced(bytes, index) {
							TokenKind::BracSpaced
						} else {
							TokenKind::BracGlued
						},
						(index, index + 1),
					));
				} else if input[index..].starts_with(']') {
					tokens.push(Token::new(TokenKind::Rbrac, (index, index + 1)));
				} else if input[index..].starts_with('{') {
					tokens.push(Token::new(TokenKind::Lbrace, (index, index + 1)));
				} else if input[index..].starts_with('}') {
					tokens.push(Token::new(TokenKind::Rbrace, (index, index + 1)));
				} else if input[index..].starts_with('?') {
					tokens.push(Token::new(TokenKind::Exists, (index, index + 1)));
				} else if input[index..].starts_with('\\') {
					tokens.push(Token::new(TokenKind::Backslash, (index, index + 1)));
				} else {
					panic!("unexpected token: {}", char);
				}
			}
			index += 1;
			continue;
		}

		if let Some(token) = &current {
			let char = bytes[index];
			match token.kind {
				TokenKind::Id => {
					if !char.is_ascii_alphabetic() && char != b'_' {
						current = tokens.next(current, index);
						index -= 1;
					}
				}
				TokenKind::Text => {
					if let Some(state) = &token.state {
						let is_escaped = state.is_escaped();
						if !is_escaped && char == b'"' {
							println!("text: {:?}", index);
							current = tokens.next(current, index + 1);
						} else {
							current = Some(token.clone_state(TokenState::new(
								Some(!is_escaped && char == b'\\'),
								None,
							)));
						}
					}
				}
				TokenKind::Quantity => {
					if !char.is_ascii_digit() {
						if char == b'.' && !token.state.as_ref().unwrap().decimals() {
							current =
								Some(token.clone_state(TokenState::new(None, Some(true))));
						} else {
							current = tokens.next(current, index);
							index -= 1;
						}
					}
				}
				_ => panic!("unknown token type: {:?}", token.kind),
			}
		}

		index += 1;
	}

	if current.is_some() {
		tokens.next(current, bytes_length);
	}

	for _ in 0..(indent.len() - 1) {
		tokens.push(Token::new(
			TokenKind::Deindent,
			(bytes_length, bytes_length),
		));
	}

	tokens.into_inner()
}

fn is_whitespace(byte: u8) -> bool {
	byte == b' '
		|| byte == b'\n'
		|| byte == b'\t'
		|| byte == b'\r'
		|| byte == b'\x0c'
		|| byte == b'\x0b'
}

fn is_spaced(bytes: &[u8], index: usize) -> bool {
	if index == 0 {
		return true;
	}
	let prev = bytes[index - 1];
	!prev.is_ascii_alphabetic() && prev != b'}'
}
