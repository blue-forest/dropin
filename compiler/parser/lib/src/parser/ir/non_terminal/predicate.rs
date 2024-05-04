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
  ir::{Expression, Logic},
  token::TokenKind,
};

use crate::parser::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  mut state: BuildState,
) -> Expression {
  state.in_keys = false;
  let first_operand = nodes[children[0]].take().unwrap().build_inner(
    #[cfg(debug_assertions)]
    stdout,
    nodes,
    input,
    state.clone(),
  );
  if children.len() > 1 {
    let mut operands = vec![first_operand];
    let mut i = 2;
    let mut logic_token = nodes[children[i - 1]].as_ref().unwrap().token;
    while i < children.len() {
      let new_logic_token = nodes[children[i - 1]].take().unwrap().token;
      if new_logic_token != logic_token {
        match logic_token {
          TokenKind::And => {
            let and = Expression::Logic(Logic::And(operands));
            operands = vec![and];
          }
          TokenKind::Or => {
            let or = Expression::Logic(Logic::Or(operands));
            operands = vec![or];
          }
          _ => unreachable!(),
        }
        logic_token = new_logic_token;
      }
      let node = nodes[children[i]].take().unwrap();
      operands.push(node.build_inner(
        #[cfg(debug_assertions)]
        stdout,
        nodes,
        input,
        state.clone(),
      ));
      i += 2;
    }
    match logic_token {
      TokenKind::And => Expression::Logic(Logic::And(operands)),
      TokenKind::Or => Expression::Logic(Logic::Or(operands)),
      _ => unreachable!(),
    }
  } else {
    first_operand
  }
}
