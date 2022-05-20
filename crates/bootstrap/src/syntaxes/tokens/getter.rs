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

use dropin_helpers::{PortableExpect, PortableUnwrap};

use crate::syntaxes::{Expression, ParseError, Patterns};

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
					query: syntax.get(start.pexpect("expected query")..).punwrap(),
				});
			}
			let (i, c) = next.punwrap();
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
						query: syntax.get(start.pexpect("expected query")..*pi).punwrap(),
					});
				}
			}
		}
	}
}

impl<'a> Token<'a> for Getter<'a> {
	fn parse<'b, 'c>(
		&self,
		patterns: &'c Patterns<'a, 'b>,
		id: &'b str,
		module: &'b str,
		iter: &mut Peekable<CharIndices<'b>>,
		expr: &mut Expression<'a, 'b>,
	) -> Result<(), ParseError<'b>> {
		if self.query.starts_with("patterns.") {
			let key = self.query.get(9..).punwrap();
			let pattern = patterns.get(key).punwrap();
			expr.add_inner(pattern.parse(patterns, id, module, iter)?);
			Ok(())
		} else if self.query.starts_with("std.") {
			if let Some(&(i, c)) = iter.peek() {
				if c.is_alphanumeric() {
					iter.next();
					Ok(())
				} else {
					err!(id, module, i, "unexpected token {}, expected alphanum", c)
				}
			} else {
				err!(id, module, module.len(),
					"unexpected end of file, expected alphanum"
				)
			}
		} else {
			err!(id, module, pos!(module, iter),
				"unknown ref: {}", self.query.to_string()
			)
		}
	}
}
