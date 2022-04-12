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

use crate::syntaxes::{Expression, Patterns, ParseError};
use super::{Concat, Token};

#[derive(Debug)]
pub struct Or<'a> {
  token1: Box<dyn Token<'a> + 'a>,
  token2: Box<dyn Token<'a> + 'a>,
}

impl<'a> Or<'a> {
  pub fn parse(
    first_token: Box<dyn Token<'a> + 'a>,
    syntax: &'a str,
    iter: &mut Peekable<CharIndices<'a>>,
  ) -> Box<dyn Token<'a> + 'a> {
    Box::new(Self{
      token1: first_token,
      token2: Concat::parse(syntax, iter),
    })
  }
}

impl<'a> Token<'a> for Or<'a> {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b>,
  ) -> Result<(), ParseError> {
    let mut iter_clone = iter.clone();
    if let Err(err1) = self.token1.parse(
      patterns, module, &mut iter_clone, expr,
    ) {
      if let Err(err2) = self.token2.parse(patterns, module, iter, expr) {
        return Err(ParseError::new(format!("{}\n{}", err1, err2)));
      }
    } else {
      *iter = iter_clone;
    }
    Ok(())
  }
}
