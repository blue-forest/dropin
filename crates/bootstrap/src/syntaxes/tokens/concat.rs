/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
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

use std::iter::Peekable;
use std::str::CharIndices;

use dropin_helpers::PortableUnwrap;

use super::{parse_token, Or, Quantifier, Token};
use crate::syntaxes::{Expression, ParseError, Patterns};

#[derive(Debug)]
pub struct Concat<'a> {
	tokens: Vec<Box<dyn Token<'a> + 'a>>,
}

impl<'a> Concat<'a> {
	pub fn parse(
		syntax: &'a str,
		iter: &mut Peekable<CharIndices<'a>>,
	) -> Box<dyn Token<'a> + 'a> {
		let mut tokens = Vec::new();
		while let Some((_, c)) = iter.next() {
			if !c.is_whitespace() {
				if c == '|' {
					return Or::parse(Box::new(Concat { tokens }), syntax, iter);
				}
				if c == ')' {
					break;
				}
				tokens.push(parse_token(syntax, iter, c));
				if let Some((_, peeked)) = iter.peek() {
					if *peeked != ')' && !peeked.is_whitespace() {
						if Quantifier::detect(*peeked) {
							let token = tokens.pop().punwrap();
							tokens.push(Box::new(Quantifier::new(syntax, iter, token)));
						} else {
							panic!("unexpected '{}'", c);
						}
					}
				}
			} else if c == '\n' {
				if let Some((_, peeked)) = iter.peek() {
					if !peeked.is_whitespace() || *peeked == '\n' {
						break;
					}
				}
			}
		}
		Box::new(Concat { tokens })
	}
}

impl<'a> Token<'a> for Concat<'a> {
	fn parse<'b, 'c>(
		&self,
		patterns: &'c Patterns<'a>,
		module: &'b str,
		iter: &mut Peekable<CharIndices<'b>>,
		expr: &mut Expression<'a, 'b>,
	) -> Result<(), ParseError> {
		for token in self.tokens.iter() {
			token.parse(patterns, module, iter, expr)?;
		}
		Ok(())
	}
}
