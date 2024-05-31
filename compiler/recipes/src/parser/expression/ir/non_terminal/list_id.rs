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

use crate::ir::Expression;
use crate::parser::expression::ir::terminal::id;
use crate::parser::expression::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  mut state: BuildState,
) -> Expression {
  let newline_position = children
    .into_iter()
    .position(|child| {
      if let TokenKind::Newline = nodes[*child].as_ref().unwrap().token {
        true
      } else {
        false
      }
    })
    .unwrap_or(children.len());
  let siblings = &children[0..newline_position];
  let key = state.value_indent_id.take().unwrap();
  let mut content = Vec::with_capacity(children.len());
  content.push(id(nodes, input, state.clone(), siblings, key));
  for i in (newline_position + 1..children.len()).step_by(2) {
    let node = nodes[children[i]].take().unwrap();
    let expr = node.build_inner(nodes, input, state.clone());
    content.push(expr);
  }
  Expression::list(content)
}
