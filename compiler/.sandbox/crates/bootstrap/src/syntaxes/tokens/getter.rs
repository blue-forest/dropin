/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
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

use std::iter::Peekable;
use std::str::CharIndices;

use crate::syntaxes::{Expression, ParseError, Patterns};
use crate::sys::{WasiExpect, WasiUnwrap};

use super::{Quantifier, Token};

#[derive(Debug)]
pub struct Getter<'a> {
	query: &'a str,
}

impl<'a> Getter<'a> {
	pub fn parse(
		syntax: &'a str,
		iter: &mut Peekable<CharIndices<'a>>,
	) -> Box<dyn Token<'a> + 'a> {
		let mut start: Option<usize> = None;
		loop {
			let next = iter.next();
			if next.is_none() {
				break Box::new(Getter {
					query: syntax
						.get(start.wasi_expect("expected query")..)
						.wasi_unwrap(),
				});
			}
			let (i, c) = next.wasi_unwrap();
			if start.is_none() {
				if !c.is_alphanumeric() {
					// $ alone ?
					panic!("unexpected query {}", c);
				}
				start = Some(i);
				continue;
			}
			if let Some((pi, pc)) = iter.peek() {
				if pc.is_whitespace() || *pc == ')' || Quantifier::detect(*pc) {
					break Box::new(Getter {
						query: syntax
							.get(start.wasi_expect("expected query")..*pi)
							.wasi_unwrap(),
					});
				}
			}
		}
	}
}

impl<'a> Token<'a> for Getter<'a> {
	fn parse<'b, 'c>(
		&self,
		patterns: &'c Patterns<'a>,
		module: &'b str,
		iter: &mut Peekable<CharIndices<'b>>,
		expr: &mut Expression<'a, 'b>,
	) -> Result<(), ParseError> {
		if self.query.starts_with("patterns.") {
			let key = self.query.get(9..).wasi_unwrap();
			let pattern = patterns.get(key).wasi_unwrap();
			expr.add_inner(pattern.parse(patterns, module, iter)?);
			Ok(())
		} else if self.query.starts_with("std.") {
			if let Some((_, c)) = iter.peek() {
				if c.is_alphanumeric() {
					iter.next();
					Ok(())
				} else {
					Err(ParseError::new(format!(
						"unexpected token {}, expected alphanum",
						c
					)))
				}
			} else {
				Err(ParseError::from(
					"unexpected end of file, expected alphanum",
				))
			}
		} else {
			return Err(ParseError::new(format!("unknown ref: {}", self.query)));
		}
	}
}
