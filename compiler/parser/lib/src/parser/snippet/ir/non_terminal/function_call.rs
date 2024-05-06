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
use dropin_compiler_common::ir::{control::FunctionCall, Control, Expression};

use crate::parser::snippet::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  state: BuildState,
) -> Expression {
  let mut args = Vec::with_capacity((children.len() - 2).div_ceil(2));
  let mut i = 1;
  while i < children.len() {
    let arg = nodes[children[i]].take().unwrap().build_inner(
      #[cfg(debug_assertions)]
      stdout,
      nodes,
      input,
      state.clone(),
    );
    args.push(arg);
    i += 2;
  }
  Expression::Control(Control::FunctionCall(FunctionCall {
    function: Box::new(state.function_call.unwrap()),
    args,
  }))
}
