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

#[cfg(debug_assertions)]
use core::fmt::Write;

use alloc::vec::Vec;
use dropin_compiler_common::{
  ir::{Expression, Value},
  token::TokenKind,
};

use super::{BuildState, ExpressionBuilder};

impl<'a> ExpressionBuilder<'a> {
  pub(super) fn build_terminal(
    self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
    state: BuildState,
    siblings: &[usize],
  ) -> Expression {
    let (start, end) = self.span.unwrap();
    let spanned_input = &input[start..end];
    match self.token {
      TokenKind::NonTerminal(_) => todo!("NonTerminal"),
      TokenKind::Newline => todo!("Newline"),
      TokenKind::Indent => todo!("Indent"),
      TokenKind::Deindent => todo!("Deindent"),
      TokenKind::ParGlued => todo!("ParGlued"),
      TokenKind::ParSpaced => todo!("ParSpaced"),
      TokenKind::BracGlued => todo!("BracGlued"),
      TokenKind::BracSpaced => todo!("BracSpaced"),
      TokenKind::If => todo!("If"),
      TokenKind::Else => todo!("Else"),
      TokenKind::True => todo!("True"),
      TokenKind::False => todo!("False"),
      TokenKind::Samekey => todo!("Samekey"),
      TokenKind::Id => {
        if state.in_keys {
          return Expression::Value(Value::Text(spanned_input.into()));
        }
        let mut indexes = Vec::with_capacity(siblings.len() / 2);
        let mut i = 1;
        let mut keys_state = state;
        keys_state.in_keys = true;
        while i < siblings.len() {
          let sep = nodes[siblings[i - 1]].take().unwrap().token;
          let key = nodes[siblings[i]].take().unwrap();
          indexes.push(key.build_inner(
            #[cfg(debug_assertions)]
            stdout,
            nodes,
            input,
            keys_state,
          ));
          if let TokenKind::BracGlued = sep {
            i += 1;
          }
          i += 2;
        }
        Expression::Value(Value::Getter(spanned_input.into(), indexes))
      }
      TokenKind::Text => todo!("Text"),
      TokenKind::Quantity => {
        Expression::Value(Value::Quantity(spanned_input.parse().unwrap()))
      }
      TokenKind::LessThan => todo!("LessThan"),
      TokenKind::MoreThan => todo!("MoreThan"),
      TokenKind::AtLeast => todo!("AtLeast"),
      TokenKind::AtMost => todo!("AtMost"),
      TokenKind::Empty => todo!("Empty"),
      TokenKind::End => todo!("End"),
      TokenKind::Eof => todo!("Eof"),
      TokenKind::Block => todo!("Block"),
      TokenKind::EqualsTo => todo!("EqualsTo"),
      TokenKind::DifferentFrom => todo!("DifferentFrom"),
      TokenKind::In => todo!("In"),
      TokenKind::Add => todo!("Add"),
      TokenKind::Sub => todo!("Sub"),
      TokenKind::Dot => todo!("Dot"),
      TokenKind::Comma => todo!("Comma"),
      TokenKind::And => todo!("And"),
      TokenKind::Or => todo!("Or"),
      TokenKind::Not => todo!("Not"),
      TokenKind::Rpar => todo!("Rpar"),
      TokenKind::Rbrac => todo!("Rbrac"),
      TokenKind::Lbrace => todo!("Lbrace"),
      TokenKind::Rbrace => todo!("Rbrace"),
      TokenKind::Exists => todo!("Exists"),
      TokenKind::Backslash => todo!("Backslash"),
    }
  }
}
