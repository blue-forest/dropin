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

mod non_terminal;
mod terminal;

#[derive(Debug)]
pub(super) struct ExpressionBuilder<'a> {
  pub(super) token: TokenKind<'a>,
  pub(super) children: Vec<usize>,
  pub(super) parent: Option<usize>,
  pub(super) span: Option<(usize, usize)>,
}

impl<'a> ExpressionBuilder<'a> {
  pub(super) fn new(
    token: TokenKind<'a>,
    parent: Option<usize>,
  ) -> ExpressionBuilder<'a> {
    ExpressionBuilder {
      token,
      children: Vec::new(),
      parent,
      span: None,
    }
  }

  pub(super) fn build(
    self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
  ) -> Expression {
    self.build_inner(stdout, nodes, input, BuildState::default())
  }

  fn build_inner(
    self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
    state: BuildState,
  ) -> Expression {
    print!(
      stdout,
      "NODES {:?}",
      nodes
        .iter()
        .map(|node| node.as_ref().map(|n| n.token.as_str()))
        .collect::<Vec<_>>()
    );
    match self.build_non_terminal(
      #[cfg(debug_assertions)]
      stdout,
      nodes,
      input,
      state,
    ) {
      Ok(expr) => {
        return expr;
      }
      Err(self_) => self_.build_terminal(
        #[cfg(debug_assertions)]
        stdout,
        nodes,
        input,
        state,
        &[],
      ),
    }
  }
}

#[derive(Default, Clone, Copy)]
struct BuildState {
  in_keys: bool,
}
