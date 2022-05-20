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

use super::{parse_token, Patterns, Token};
use crate::syntaxes::{Expression, ParseError};

#[derive(Debug)]
pub struct Not<'a> {
	token: Box<dyn Token<'a> + 'a>,
}

impl<'a> Not<'a> {
	pub fn parse(
		syntax: &'a str,
		iter: &mut Peekable<CharIndices<'a>>,
	) -> Box<dyn Token<'a> + 'a> {
		while let Some((_, c)) = iter.next() {
			if !c.is_whitespace() {
				return Box::new(Self {
					token: parse_token(syntax, iter, c),
				});
			}
		}
		panic!("unexpected end of file");
	}
}

impl<'a> Token<'a> for Not<'a> {
	fn parse<'b, 'c>(
		&self,
		patterns: &'c Patterns<'a, 'b>,
		module: &'b str,
		iter: &mut Peekable<CharIndices<'b>>,
		expr: &mut Expression<'a, 'b>,
	) -> Result<(), ParseError<'b>> {
		let mut iter_clone = iter.clone();
		if let Ok(()) = self.token.parse(patterns, module, &mut iter_clone, expr) {
			err!(module, pos!(module, iter),
				"expected not {}", self.token.expected()
			)
		} else {
			iter.next();
			Ok(())
		}
	}
}
