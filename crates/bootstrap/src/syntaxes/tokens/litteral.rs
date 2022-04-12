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
use super::Token;

#[derive(Debug)]
pub struct Litteral<'a> {
  value: &'a str,
}

impl<'a> Litteral<'a> {
  pub fn parse(
    syntax: &'a str,
    iter: &mut Peekable<CharIndices<'a>>,
  ) -> Box<dyn Token<'a> + 'a> {
    let mut start: Option<usize> = None;
    let mut value: Option<&str> = None;
    let mut is_escaped = false;
    while let Some((i, c)) = iter.next() {
      if start.is_none() {
        start = Some(i);
      }
      if !is_escaped {
        match c {
          '"' => {
            value = Some(syntax.get(start.unwrap()..i).unwrap());
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
    let value = value.expect("expected '\"'");
    Box::new(Litteral{ value })
  }
}

impl<'a> Token<'a> for Litteral<'a> {
  fn parse<'b, 'c>(
    &self,
    _patterns: &'c Patterns<'a>,
    _module:   &'b str,
    iter:      &mut Peekable<CharIndices<'b>>,
    _expr:     &mut Expression,
  ) -> Result<(), ParseError> {
    let mut is_escaped = false;
    for chr_value in self.value.chars() {
      if !is_escaped && chr_value == '\\' {
        is_escaped = true;
        continue;
      }
      let ok = if let Some((_, chr_module)) = iter.peek() {
        if *chr_module == chr_value { iter.next(); true } else { false }
      } else { false };
      if !ok {
        return Err(ParseError::new(format!("expected {}", self.value)));
      }
      is_escaped = false;
    }
    Ok(())
  }
}

