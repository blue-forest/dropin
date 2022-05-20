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

use super::Token;
use crate::syntaxes::{Expression, ParseError, Patterns};
use crate::utils::escape_char;

#[derive(Debug)]
pub struct Literal<'a> {
	value: &'a str,
}

impl<'a> Literal<'a> {
	pub fn parse(
		syntax: &'a str,
		iter: &mut Peekable<CharIndices<'a>>,
	) -> Box<dyn Token<'a> + 'a> {
		let mut start: Option<usize> = None;
		let mut value: Option<&str> = None;
		let mut is_escaped = false;
		for (i, c) in iter {
			if start.is_none() {
				start = Some(i);
			}
			if !is_escaped {
				match c {
					'"' => {
						value = Some(syntax.get(start.punwrap()..i).punwrap());
						break;
					}
					'\\' => {
						is_escaped = true;
						continue;
					}
					_ => {}
				}
			}
			is_escaped = false;
		}
		let value = value.pexpect("expected '\"'");
		Box::new(Literal { value })
	}
}

impl<'a> Token<'a> for Literal<'a> {
	fn parse<'b, 'c>(
		&self,
		_patterns: &'c Patterns<'a, 'b>,
		module: &'b str,
		iter: &mut Peekable<CharIndices<'b>>,
		_expr: &mut Expression,
	) -> Result<(), ParseError<'b>> {
		let mut is_escaped = false;
		for c in self.value.chars() {
			if !is_escaped && c == '\\' {
				is_escaped = true;
				continue;
			}
			let chr_value = if is_escaped { escape_char(c) } else { c };
			let ok = if let Some((_, chr_module)) = iter.peek() {
				if *chr_module == chr_value {
					iter.next();
					true
				} else {
					false
				}
			} else {
				false
			};
			if !ok {
				return err!(
					module, pos!(module, iter), "expected {}", self.value.to_string()
				)
			}
			is_escaped = false;
		}
		Ok(())
	}
}
