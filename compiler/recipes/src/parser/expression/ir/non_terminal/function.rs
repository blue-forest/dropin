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
  mut state: BuildState,
) -> Expression {
  let args_children = nodes[children[1]].take().unwrap().children;
  let mut i = 0;
  let mut args = Vec::with_capacity(args_children.len().div_ceil(2));
  while i < args_children.len() {
    let arg_child = args_children[i];
    let (start, end) = nodes[arg_child].take().unwrap().span.unwrap();
    let arg = &input[start..end];
    args.push(arg.into());
    i += 2;
  }
  let body =
    nodes[children[3]]
      .take()
      .unwrap()
      .build_inner(nodes, input, state.clone());
  let function = if let Some(name) = state.function_name {
    Expression::named_function(name.into(), args, body)
  } else {
    Expression::anonymous_function(args, body)
  };
  if children.len() > 5 {
    state.function_call = Some(function);
    nodes[children[5]]
      .take()
      .unwrap()
      .build_inner(nodes, input, state.clone())
  } else {
    function
  }
}
