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
  ir::{control::If, Comparison, Control, Expression, Logic},
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
      "predicate" => {
        let first_operand =
          nodes[self.children[0]].take().unwrap().build_inner(
            #[cfg(debug_assertions)]
            stdout,
            nodes,
            input,
            state,
          );
        if self.children.len() > 1 {
          let mut operands = vec![first_operand];
          let mut i = 2;
          let mut logic_token =
            nodes[self.children[i - 1]].as_ref().unwrap().token;
          while i < self.children.len() {
            let new_logic_token =
              nodes[self.children[i - 1]].take().unwrap().token;
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
            let node = nodes[self.children[i]].take().unwrap();
            operands.push(node.build_inner(
              #[cfg(debug_assertions)]
              stdout,
              nodes,
              input,
              state,
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
      "if" => {
        let condition = nodes[self.children[1]].take().unwrap().build_inner(
          #[cfg(debug_assertions)]
          stdout,
          nodes,
          input,
          state,
        );
        let then = nodes[self.children[3]].take().unwrap().build_inner(
          #[cfg(debug_assertions)]
          stdout,
          nodes,
          input,
          state,
        );
        let mut else_ = None;
        if self.children.len() > 4 {
          else_ = Some(Box::new(
            nodes[self.children[4]].take().unwrap().build_inner(
              #[cfg(debug_assertions)]
              stdout,
              nodes,
              input,
              state,
            ),
          ))
        }
        Expression::Control(Control::If(If {
          condition: Box::new(condition),
          then: Box::new(then),
          else_,
        }))
      }
      "if-else" => nodes[self.children[2]].take().unwrap().build_inner(
        #[cfg(debug_assertions)]
        stdout,
        nodes,
        input,
        state,
      ),
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
