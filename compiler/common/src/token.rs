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
  Terminal(&'a str),
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
      Self::Terminal(name) => name,
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
      Self::Terminal(value) => {
        quote!(dropin_compiler_common::token::TokenKind::Terminal(#value))
      }
      Self::NonTerminal(value) => {
        quote!(dropin_compiler_common::token::TokenKind::NonTerminal(#value))
      }
      Self::Newline => {
        quote!(dropin_compiler_common::token::TokenKind::Newline)
      }
      Self::Indent => quote!(dropin_compiler_common::token::TokenKind::Indent),
      Self::Deindent => {
        quote!(dropin_compiler_common::token::TokenKind::Deindent)
      }
      Self::ParGlued => {
        quote!(dropin_compiler_common::token::TokenKind::ParGlued)
      }
      Self::ParSpaced => {
        quote!(dropin_compiler_common::token::TokenKind::ParSpaced)
      }
      Self::BracGlued => {
        quote!(dropin_compiler_common::token::TokenKind::BracGlued)
      }
      Self::BracSpaced => {
        quote!(dropin_compiler_common::token::TokenKind::BracSpaced)
      }
      Self::If => quote!(dropin_compiler_common::token::TokenKind::If),
      Self::Else => quote!(dropin_compiler_common::token::TokenKind::Else),
      Self::True => quote!(dropin_compiler_common::token::TokenKind::True),
      Self::False => quote!(dropin_compiler_common::token::TokenKind::False),
      Self::Samekey => {
        quote!(dropin_compiler_common::token::TokenKind::Samekey)
      }
      Self::Id => quote!(dropin_compiler_common::token::TokenKind::Id),
      Self::Text => quote!(dropin_compiler_common::token::TokenKind::Text),
      Self::Quantity => {
        quote!(dropin_compiler_common::token::TokenKind::Quantity)
      }
      Self::LessThan => {
        quote!(dropin_compiler_common::token::TokenKind::LessThan)
      }
      Self::MoreThan => {
        quote!(dropin_compiler_common::token::TokenKind::MoreThan)
      }
      Self::AtLeast => {
        quote!(dropin_compiler_common::token::TokenKind::AtLeast)
      }
      Self::AtMost => quote!(dropin_compiler_common::token::TokenKind::AtMost),
      Self::Empty => quote!(dropin_compiler_common::token::TokenKind::Empty),
      Self::End => quote!(dropin_compiler_common::token::TokenKind::End),
      Self::Eof => quote!(dropin_compiler_common::token::TokenKind::Eof),
      Self::Block => quote!(dropin_compiler_common::token::TokenKind::Block),
      Self::EqualsTo => {
        quote!(dropin_compiler_common::token::TokenKind::EqualsTo)
      }
      Self::DifferentFrom => {
        quote!(dropin_compiler_common::token::TokenKind::DifferentFrom)
      }
      Self::In => quote!(dropin_compiler_common::token::TokenKind::In),
      Self::Add => quote!(dropin_compiler_common::token::TokenKind::Add),
      Self::Sub => quote!(dropin_compiler_common::token::TokenKind::Sub),
      Self::Dot => quote!(dropin_compiler_common::token::TokenKind::Dot),
      Self::Comma => quote!(dropin_compiler_common::token::TokenKind::Comma),
      Self::And => quote!(dropin_compiler_common::token::TokenKind::And),
      Self::Or => quote!(dropin_compiler_common::token::TokenKind::Or),
      Self::Not => quote!(dropin_compiler_common::token::TokenKind::Not),
      Self::Rpar => quote!(dropin_compiler_common::token::TokenKind::Rpar),
      Self::Rbrac => quote!(dropin_compiler_common::token::TokenKind::Rbrac),
      Self::Lbrace => quote!(dropin_compiler_common::token::TokenKind::Lbrace),
      Self::Rbrace => quote!(dropin_compiler_common::token::TokenKind::Rbrace),
      Self::Exists => quote!(dropin_compiler_common::token::TokenKind::Exists),
      Self::Backslash => {
        quote!(dropin_compiler_common::token::TokenKind::Backslash)
      }
    };
    tokens.extend(extension);
  }
}
