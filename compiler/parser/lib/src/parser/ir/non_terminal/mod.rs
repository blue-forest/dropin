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
use dropin_compiler_common::{ir::Expression, token::TokenKind};

use super::{BuildState, ExpressionBuilder};

mod expression;
mod function;
mod function_call;
mod if_;
mod if_else;
mod if_then;
mod list_lit;
mod predicate;
mod value_lit;
mod value_no_indent;

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
    macro_rules! build {
      ($($pat:pat => $module:ident),*) => {
        match non_terminal {
          $(
            $pat => $module::build(
              #[cfg(debug_assertions)]
              stdout,
              &self.children,
              nodes,
              input,
              state,
            ),
          )*
          _ => {
            assert!(
              self.children.len() == 1,
              "{non_terminal} has several children\n{:?}",
              self.children.iter().map(|i| nodes[*i].as_ref().unwrap().token).collect::<Vec<_>>()
            );
            nodes[self.children[0]].take().unwrap().build_inner(
              #[cfg(debug_assertions)]
              stdout,
              nodes,
              input,
              state,
            )
          }
        }
      };
    }
    Ok(build!(
      "predicate" => predicate,
      "expression" => expression,
      "value-no-indent" => value_no_indent,
      "value-lit" => value_lit,
      "list-lit" => list_lit,
      "anonymous-function" => function,
      "function-call" => function_call,
      "if" => if_,
      "if-then" => if_then,
      "if-else" => if_else
    ))
  }
}
