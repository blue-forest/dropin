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

use std::vec::Vec;

use dropin_compiler_common::TokenKind;

use crate::ir::{
  expression::ExpressionInner, value::ValueInner, Expression, RichTextPart,
  Value,
};

use super::{BuildState, ExpressionBuilder};

impl<'a> ExpressionBuilder<'a> {
  pub(super) fn build_terminal(
    self,
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
    state: BuildState,
    siblings: &[usize],
  ) -> Expression {
    let (start, end) = self.span.unwrap();
    let spanned_input = &input[start..end];
    match self.token {
      TokenKind::BracSpaced => Expression::list(vec![]),
      TokenKind::True => Expression::boolean(true),
      TokenKind::False => Expression::boolean(false),
      TokenKind::Id => id(nodes, input, state, siblings, spanned_input),
      TokenKind::Text => Expression {
        expression_inner: Some(ExpressionInner::Value(Value {
          value_inner: Some(ValueInner::Text(
            serde_yaml::from_str(spanned_input.trim_matches('"')).unwrap(),
          )),
        })),
      },
      TokenKind::Quantity => {
        Expression::quantity(spanned_input.parse().unwrap())
      }
      _ => unreachable!("{:?}", self.token),
    }
  }
}

pub fn id(
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  state: BuildState,
  siblings: &[usize],
  key: &str,
) -> Expression {
  if state.in_keys {
    return Expression::text(vec![RichTextPart::r#static(key.into())]);
  }
  if !siblings.is_empty() {
    let continuation_token = nodes[siblings[0]].as_ref().unwrap().token;
    match continuation_token {
      // named function
      TokenKind::Lbrace => {
        let mut function_state = state;
        function_state.function_name = Some(key);
        let node = nodes[siblings[1]].take().unwrap();
        return node.build_inner(nodes, input, function_state);
      }
      TokenKind::NonTerminal("function-call") => {
        let mut call_state = state;
        call_state.function_call = Some(Expression::getter(key.into(), vec![]));
        return nodes[siblings[0]]
          .take()
          .unwrap()
          .build_non_terminal(nodes, input, call_state)
          .unwrap();
      }
      _ => {}
    }
  }
  let mut indexes = Vec::with_capacity(siblings.len() / 2);
  let mut i = 1;
  let mut keys_state = state.clone();
  keys_state.in_keys = true;
  while i < siblings.len() {
    let sep = nodes[siblings[i - 1]].take().unwrap().token;
    let key = nodes[siblings[i]].take().unwrap();
    indexes.push(key.build_inner(nodes, input, keys_state.clone()));
    if let TokenKind::BracGlued = sep {
      i += 1;
    }
    i += 2;
  }
  Expression::getter(key.into(), indexes)
}
