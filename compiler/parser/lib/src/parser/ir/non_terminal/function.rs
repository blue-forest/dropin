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

use alloc::{boxed::Box, vec::Vec};
use dropin_compiler_common::ir::{
  control::{AnonymousFunction, NamedFunction},
  Control, Expression,
};

use crate::parser::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  mut state: BuildState,
) -> Expression {
  let args_children = nodes[children[0]].take().unwrap().children;
  let mut i = 0;
  let mut args = Vec::with_capacity(args_children.len().div_ceil(2));
  while i < args_children.len() {
    let arg_child = args_children[i];
    let (start, end) = nodes[arg_child].take().unwrap().span.unwrap();
    let arg = &input[start..end];
    args.push(arg.into());
    i += 2;
  }
  let body = nodes[children[2]].take().unwrap().build_inner(
    #[cfg(debug_assertions)]
    stdout,
    nodes,
    input,
    state.clone(),
  );
  let function = if let Some(name) = state.function_name {
    Expression::Control(Control::NamedFunction(NamedFunction {
      name: name.into(),
      args,
      body: Box::new(body),
    }))
  } else {
    Expression::Control(Control::AnonymousFunction(AnonymousFunction {
      args,
      body: Box::new(body),
    }))
  };
  if children.len() > 4 {
    state.function_call = Some(function);
    nodes[children[4]].take().unwrap().build_inner(
      #[cfg(debug_assertions)]
      stdout,
      nodes,
      input,
      state.clone(),
    )
  } else {
    function
  }
}
