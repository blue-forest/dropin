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
  ir::{Comparison, Expression},
  token::TokenKind,
};

use super::{BuildState, ExpressionBuilder};

impl<'a> ExpressionBuilder<'a> {
  pub(super) fn build_non_terminal(
    self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
    state: BuildState,
  ) -> Result<Expression, ExpressionBuilder<'a>> {
    let TokenKind::NonTerminal(non_terminal) = self.token else {
      return Err(self);
    };
    Ok(match non_terminal {
      "expression" => {
        let left = nodes[self.children[0]].take().unwrap().build_inner(
          #[cfg(debug_assertions)]
          stdout,
          nodes,
          input,
          state,
        );
        if self.children.len() > 1 {
          let continuation_token =
            nodes[self.children[1]].take().unwrap().token;
          match continuation_token {
            TokenKind::EqualsTo => {
              let right = nodes[self.children[2]].take().unwrap().build_inner(
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
      "value-no-indent" => {
        let node = nodes[self.children[0]].take().unwrap();
        let first_token = &node.token;
        match first_token {
          // value-lit
          TokenKind::NonTerminal(_) => node.build_inner(
            #[cfg(debug_assertions)]
            stdout,
            nodes,
            input,
            state,
          ),
          TokenKind::Id => node.build_terminal(
            #[cfg(debug_assertions)]
            stdout,
            nodes,
            input,
            state,
            &self.children[1..],
          ),
          TokenKind::Exists => todo!(),
          TokenKind::Not => todo!(),
          _ => unreachable!(),
        }
      }
      _ => {
        // if self.children.len() != 1 {
        //   print!(
        //     stdout,
        //     "=================\n{:?}",
        //     self
        //       .children
        //       .iter()
        //       .map(|i| format!("{:?}", nodes[*i].as_ref().unwrap().token))
        //       .collect::<Vec<_>>()
        //   );
        // }
        assert!(
          self.children.len() == 1,
          "{non_terminal} has several children"
        );
        nodes[self.children[0]].take().unwrap().build_inner(
          #[cfg(debug_assertions)]
          stdout,
          nodes,
          input,
          state,
        )
      }
    })
  }
}
