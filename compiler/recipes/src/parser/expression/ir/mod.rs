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

use dropin_compiler_common::TokenKind;
use std::vec::Vec;

use crate::ir::Expression;

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
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
  ) -> Expression {
    self.build_inner(nodes, input, BuildState::default())
  }

  fn build_inner(
    self,
    nodes: &mut Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
    state: BuildState,
  ) -> Expression {
    match self.build_non_terminal(nodes, input, state.clone()) {
      Ok(expr) => {
        return expr;
      }
      Err(self_) => self_.build_terminal(nodes, input, state, &[]),
    }
  }

  #[cfg(debug_assertions)]
  pub(super) fn debug(
    &self,
    nodes: &Vec<Option<ExpressionBuilder<'a>>>,
    input: &str,
    indent: usize,
  ) {
    if let TokenKind::NonTerminal(name) = self.token {
      debug!("{name:>0$}", name.len() + indent * 2);
      for child in &self.children {
        nodes[*child]
          .as_ref()
          .unwrap()
          .debug(nodes, input, indent + 1);
      }
    } else {
      let mut name = format!("{:?}", self.token);
      if let Some((start, end)) = self.span {
        name.push(' ');
        name.push_str(&input[start..end]);
      }
      debug!("{name:>0$}", name.len() + indent * 2);
    }
  }
}

#[derive(Default, Clone)]
struct BuildState<'a> {
  in_keys: bool,
  function_name: Option<&'a str>,
  function_call: Option<Expression>,
}
