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
use dropin_compiler_common::{
  ir::{Arithmetic, Comparison, Expression},
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
    state.clone(),
  );
  if children.len() > 1 {
    let continuation_token = nodes[children[1]].take().unwrap().token;
    macro_rules! binary {
        ($($pat:ident => $expr:ident::$variant:ident),*) => {
          match continuation_token {
                $(TokenKind::$pat => {
                  let right = nodes[children[2]].take().unwrap().build_inner(
                    #[cfg(debug_assertions)]
                    stdout,
                    nodes,
                    input,
                    state,
                  );
                  Expression::$expr($expr::$variant(
                    Box::new(left),
                    Box::new(right),
                  ))
                })*
            _ => {
              panic!("unknown expression continuation: {continuation_token:?}")
            }
          }
        };
    }
    binary!(
      EqualsTo => Comparison::EqualsTo,
      DifferentFrom => Comparison::DifferentFrom,
      In => Comparison::In,
      LessThan => Comparison::LessThan,
      MoreThan => Comparison::MoreThan,
      AtLeast => Comparison::AtLeast,
      AtMost => Comparison::AtMost,
      Add => Arithmetic::Add,
      Sub => Arithmetic::Sub
    )
  } else {
    left
  }
}
