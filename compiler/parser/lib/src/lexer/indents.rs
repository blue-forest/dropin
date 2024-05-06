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

use alloc::vec::Vec;
use dropin_compiler_common::token::TokenKind;

pub struct IndentLexer {
  indents: Vec<usize>,
}

impl Default for IndentLexer {
  fn default() -> Self {
    Self { indents: vec![0] }
  }
}

impl IndentLexer {
  pub fn parse(&mut self, input: &[u8]) -> (usize, IndentKind) {
    let mut line_indent = 0;
    let mut index = 0;
    while index < input.len() {
      let char = input[index];
      if char == b' ' {
        line_indent += 1;
      } else if char == b'\t' {
        line_indent += 2;
      } else {
        if self.indents[self.indents.len() - 1] < line_indent {
          self.indents.push(line_indent);
          return (index, IndentKind::Indent);
        }
        if self.indents[self.indents.len() - 1] > line_indent {
          self.indents.pop();
          return (index, IndentKind::Deindent);
        }
        return (index, IndentKind::Newline);
      }
      index += 1;
    }
    (index, IndentKind::Eof)
  }

  pub fn len(&self) -> usize {
    self.indents.len()
  }
}

pub enum IndentKind {
  Indent,
  Deindent,
  Newline,
  Eof,
}

impl<'a> Into<TokenKind<'a>> for IndentKind {
  fn into(self) -> TokenKind<'a> {
    match self {
      Self::Indent => TokenKind::Indent,
      Self::Deindent => TokenKind::Deindent,
      Self::Newline => TokenKind::Newline,
      Self::Eof => TokenKind::Eof,
    }
  }
}
