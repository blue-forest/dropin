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

#![no_std]

#[cfg(test)]
extern crate std;

#[macro_use]
extern crate alloc;

pub use crate::lexer::lexer;
pub use crate::parser::parse;
pub use crate::token::Token;

macro_rules! print {
  ($stdout:ident, $($arg:tt)*) => {
    #[cfg(debug_assertions)]
    let _ = writeln!($stdout, $($arg)*);
  };
}

mod lexer;
mod parser;
mod token;

#[dropin_compiler_parser_macros::table(
  grammar = "compiler/parser/lib/src/grammar.abnf"
)]
pub struct Table;