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

use core::fmt::Write;

use alloc::{boxed::Box, vec::Vec};
use dropin_compiler_common::{
  ir::{Comparison, Expression},
  token::TokenKind,
};

use crate::parser::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  state: BuildState,
) -> Expression {
  let left = nodes[children[0]].take().unwrap().build_inner(
    #[cfg(debug_assertions)]
    stdout,
    nodes,
    input,
    state,
  );
  if children.len() > 1 {
    let continuation_token = nodes[children[1]].take().unwrap().token;
    match continuation_token {
      TokenKind::EqualsTo => {
        let right = nodes[children[2]].take().unwrap().build_inner(
          #[cfg(debug_assertions)]
          stdout,
          nodes,
          input,
          state,
        );
        Expression::Comparison(Comparison::EqualsTo(
          Box::new(left),
          Box::new(right),
        ))
      }
      _ => {
        panic!("unknown expression continuation: {continuation_token:?}")
      }
    }
  } else {
    left
  }
}
