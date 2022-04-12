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

use std::fmt::Debug;
use std::iter::Peekable;
use std::str::CharIndices;

use super::{Expression, Patterns, ParseError};

mod concat;
pub use concat::Concat;

mod getter;
pub use getter::Getter;

mod litteral;
pub use litteral::Litteral;

mod not;
pub use not::Not;

mod or;
pub use or::Or;

mod quantifier;
pub use quantifier::Quantifier;

pub trait Token<'a>: Debug {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b>,
  ) -> Result<(), ParseError>;

  fn expected(&self) -> String {
    "unknown".to_string()
  }
}

fn parse_token<'a>(
  syntax: &'a str,
  iter: &mut Peekable<CharIndices<'a>>,
  c: char,
) -> Box<dyn Token<'a> + 'a> {
  match c {
    '"' => Litteral::parse(syntax, iter),
    '$' => Getter::parse(syntax, iter),
    '!' => Not::parse(syntax, iter),
    '(' => Concat::parse(syntax, iter),
    _   => { panic!("unknown token {}", c); }
  }
}

