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

use crate::ir::Expression;
use crate::parser::expression::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  state: BuildState,
) -> Expression {
  let condition =
    nodes[children[1]]
      .take()
      .unwrap()
      .build_inner(nodes, input, state.clone());
  let then =
    nodes[children[3]]
      .take()
      .unwrap()
      .build_inner(nodes, input, state.clone());
  let mut else_ = None;
  if children.len() > 4 {
    let else_node = nodes[children[4]].take().unwrap();
    if !else_node.children.is_empty() {
      else_ = Some(else_node.build_inner(nodes, input, state))
    }
  }
  Expression::r#if(condition, then, else_)
}