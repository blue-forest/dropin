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

use crate::syntaxes::{Expression, ParseError, Patterns};
use super::Token;

#[derive(Debug)]
pub enum Quantifier<'a> {
  AtLeastOne(Box<dyn Token<'a> + 'a>),
}

impl<'a> Quantifier<'a> {
  pub fn detect(c: char) -> bool {
    c == '+'
  }

  pub fn new(
    iter:   &mut Peekable<CharIndices<'a>>,
    expr:   Box<dyn Token<'a> + 'a>
  ) -> Self {
    let (_, c) = iter.next().unwrap();
    match c {
      '+' => Self::AtLeastOne(expr),
      _   => { unreachable!() }
    }
  }
}

impl<'a> Token<'a> for Quantifier<'a> {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b>,
  ) -> Result<(), ParseError> {
    match self {
      Self::AtLeastOne(token) => {
        let mut ok = false;
        let err = loop {
          if let Err(err) = token.parse(patterns, module, iter, expr) {
            break Err(err);
          }
          ok = true;
          if let None = iter.peek() {
            break Ok(())
          }
        };
        if !ok { err } else { Ok(()) }
      }
    }
  }
}
