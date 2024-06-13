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

#[cfg(feature = "macros")]
use proc_macro2::TokenStream;
#[cfg(feature = "macros")]
use quote::{quote, ToTokens};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum TokenKind<'a> {
  NonTerminal(&'a str),
  Newline,
  Indent,
  Deindent,
  ParGlued,
  ParSpaced,
  BracGlued,
  BracSpaced,
  If,
  Else,
  True,
  False,
  Samekey,
  Id,
  Text,
  Quantity,
  LessThan,
  MoreThan,
  AtLeast,
  AtMost,
  Empty,
  End,
  Eof,
  Block,
  EqualsTo,
  DifferentFrom,
  In,
  Add,
  Sub,
  Dot,
  Comma,
  And,
  Or,
  Not,
  Rpar,
  Rbrac,
  Lbrace,
  Rbrace,
  Exists,
  Backslash,
}

impl<'a> TokenKind<'a> {
  pub fn as_str(&self) -> &'a str {
    match self {
      Self::NonTerminal(name) => name,
      Self::Newline => "NEWLINE",
      Self::Indent => "INDENT",
      Self::Deindent => "DEINDENT",
      Self::ParGlued => "PARGLUED",
      Self::ParSpaced => "PARSPACED",
      Self::BracGlued => "BRACGLUED",
      Self::BracSpaced => "BRACSPACED",
      Self::If => "IF",
      Self::Else => "ELSE",
      Self::True => "TRUE",
      Self::False => "FALSE",
      Self::Samekey => "SAMEKEY",
      Self::Id => "ID",
      Self::Text => "TEXT",
      Self::Quantity => "QUANTITY",
      Self::LessThan => "LESSTHAN",
      Self::MoreThan => "MORETHAN",
      Self::AtLeast => "ATLEAST",
      Self::AtMost => "ATMOST",
      Self::Empty => "EMPTY",
      Self::End => "END",
      Self::Eof => "EOF",

      Self::Block => ":",
      Self::EqualsTo => "==",
      Self::DifferentFrom => "!=",
      Self::In => "in",
      Self::Add => "+",
      Self::Sub => "-",
      Self::Dot => ".",
      Self::Comma => ",",
      Self::And => "&",
      Self::Or => "|",
      Self::Not => "!",
      Self::Rpar => ")",
      Self::Rbrac => "]",
      Self::Lbrace => "{",
      Self::Rbrace => "}",
      Self::Exists => "?",
      Self::Backslash => "\\",
    }
  }
}

#[cfg(feature = "macros")]
impl<'a> ToTokens for TokenKind<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let extension = match self {
      Self::NonTerminal(value) => {
        quote!(dropin_compiler_common::TokenKind::NonTerminal(#value))
      }
      Self::Newline => {
        quote!(dropin_compiler_common::TokenKind::Newline)
      }
      Self::Indent => quote!(dropin_compiler_common::TokenKind::Indent),
      Self::Deindent => {
        quote!(dropin_compiler_common::TokenKind::Deindent)
      }
      Self::ParGlued => {
        quote!(dropin_compiler_common::TokenKind::ParGlued)
      }
      Self::ParSpaced => {
        quote!(dropin_compiler_common::TokenKind::ParSpaced)
      }
      Self::BracGlued => {
        quote!(dropin_compiler_common::TokenKind::BracGlued)
      }
      Self::BracSpaced => {
        quote!(dropin_compiler_common::TokenKind::BracSpaced)
      }
      Self::If => quote!(dropin_compiler_common::TokenKind::If),
      Self::Else => quote!(dropin_compiler_common::TokenKind::Else),
      Self::True => quote!(dropin_compiler_common::TokenKind::True),
      Self::False => quote!(dropin_compiler_common::TokenKind::False),
      Self::Samekey => {
        quote!(dropin_compiler_common::TokenKind::Samekey)
      }
      Self::Id => quote!(dropin_compiler_common::TokenKind::Id),
      Self::Text => quote!(dropin_compiler_common::TokenKind::Text),
      Self::Quantity => {
        quote!(dropin_compiler_common::TokenKind::Quantity)
      }
      Self::LessThan => {
        quote!(dropin_compiler_common::TokenKind::LessThan)
      }
      Self::MoreThan => {
        quote!(dropin_compiler_common::TokenKind::MoreThan)
      }
      Self::AtLeast => {
        quote!(dropin_compiler_common::TokenKind::AtLeast)
      }
      Self::AtMost => quote!(dropin_compiler_common::TokenKind::AtMost),
      Self::Empty => quote!(dropin_compiler_common::TokenKind::Empty),
      Self::End => quote!(dropin_compiler_common::TokenKind::End),
      Self::Eof => quote!(dropin_compiler_common::TokenKind::Eof),
      Self::Block => quote!(dropin_compiler_common::TokenKind::Block),
      Self::EqualsTo => {
        quote!(dropin_compiler_common::TokenKind::EqualsTo)
      }
      Self::DifferentFrom => {
        quote!(dropin_compiler_common::TokenKind::DifferentFrom)
      }
      Self::In => quote!(dropin_compiler_common::TokenKind::In),
      Self::Add => quote!(dropin_compiler_common::TokenKind::Add),
      Self::Sub => quote!(dropin_compiler_common::TokenKind::Sub),
      Self::Dot => quote!(dropin_compiler_common::TokenKind::Dot),
      Self::Comma => quote!(dropin_compiler_common::TokenKind::Comma),
      Self::And => quote!(dropin_compiler_common::TokenKind::And),
      Self::Or => quote!(dropin_compiler_common::TokenKind::Or),
      Self::Not => quote!(dropin_compiler_common::TokenKind::Not),
      Self::Rpar => quote!(dropin_compiler_common::TokenKind::Rpar),
      Self::Rbrac => quote!(dropin_compiler_common::TokenKind::Rbrac),
      Self::Lbrace => quote!(dropin_compiler_common::TokenKind::Lbrace),
      Self::Rbrace => quote!(dropin_compiler_common::TokenKind::Rbrace),
      Self::Exists => quote!(dropin_compiler_common::TokenKind::Exists),
      Self::Backslash => {
        quote!(dropin_compiler_common::TokenKind::Backslash)
      }
    };
    tokens.extend(extension);
  }
}

pub fn to_upper_camelcase(id: &str) -> String {
  let mut result = String::new();
  let mut is_capital = true;
  for c in id.chars() {
    if c == '_' || c == '/' {
      is_capital = true;
      continue;
    }
    if is_capital {
      result.push(c.to_ascii_uppercase());
    } else {
      result.push(c);
    }
    is_capital = false;
  }
  result
}

#[derive(Debug)]
pub enum Key<'a> {
  Text(&'a str),
  Quantity(usize),
}
