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

use alloc::fmt::{Debug, Formatter};

use alloc::vec::Vec;
use dropin_compiler_common::ir::Expression;
use dropin_compiler_common::token::TokenKind;

use crate::parser::snippet::ir::ExpressionBuilder;

pub(super) struct Stack<'a> {
  nodes: Vec<Option<ExpressionBuilder<'a>>>,
  stack: Vec<usize>,
}

impl Debug for Stack<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> alloc::fmt::Result {
    // let mut first = true;
    // write!(f, "ALL NODES\n")?;
    // for node in &self.nodes {
    //   if !first {
    //     write!(f, ", ")?;
    //   }
    //   write!(f, "{:?}", node.as_ref().unwrap().token.as_str())?;
    //   first = false;
    // }
    write!(f, "\nCURRENT\n")?;
    let mut first = true;
    for i in &self.stack {
      let node = &self.nodes[*i];
      if !first {
        write!(f, ", ")?;
      }
      write!(f, "{:?}", node.as_ref().unwrap().token.as_str())?;
      first = false;
    }
    Ok(())
  }
}

impl<'a> Stack<'a> {
  pub(super) fn new(main_non_terminal: &'a str) -> Stack<'a> {
    Stack {
      nodes: vec![
        Some(ExpressionBuilder::new(TokenKind::NonTerminal("root"), None)),
        Some(ExpressionBuilder::new(
          TokenKind::NonTerminal(main_non_terminal),
          Some(0),
        )),
      ],
      stack: vec![0, 1],
    }
  }

  // pub(super) fn push(&mut self, node: NodeBuilder<'a>) {
  //   self.nodes.push(node.into());
  //   self.stack.push(self.nodes.len() - 1);
  // }

  pub(super) fn pop<'s>(&'s mut self) -> StackNode<'a, 's> {
    let i = self.stack.pop().unwrap();
    StackNode { stack: self, i }
  }

  pub(super) fn is_empty(&self) -> bool {
    self.stack.len() <= 1
  }

  pub(super) fn into_expression(
    mut self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    input: &str,
  ) -> Expression {
    let i = self.nodes[0].as_mut().unwrap().children.pop().unwrap();
    self.nodes[i].take().unwrap().build(
      #[cfg(debug_assertions)]
      stdout,
      &mut self.nodes,
      input,
    )
  }

  pub(super) fn push_children(
    &mut self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    i: usize,
  ) {
    let node = self.nodes[i].as_ref().unwrap();
    let parent = node.parent.unwrap();
    print!(
      stdout,
      "PUSH_CHILDREN {:?} {:?}",
      node.token.as_str(),
      parent
    );
    self.nodes[parent].as_mut().unwrap().children.push(i);
  }

  pub(super) fn pop_children(&mut self, parent: usize) {
    self.nodes[parent].as_mut().unwrap().children.pop();
  }

  pub(super) fn substitute(
    &mut self,
    #[cfg(debug_assertions)] stdout: &mut impl Write,
    mut parent: Option<usize>,
    substitute: &[TokenKind<'a>],
  ) {
    while let Some(new_parent) = parent {
      // 	print!(stdout, "NEW_PARENT {:?}", new_parent);
      let TokenKind::NonTerminal(token) =
        self.nodes[new_parent].as_ref().unwrap().token
      else {
        unreachable!()
      };
      if !token.ends_with("-")
        || self.nodes[new_parent].as_ref().unwrap().parent.is_none()
      {
        break;
      }
      parent = self.nodes[new_parent].as_ref().unwrap().parent;
    }
    print!(stdout, "SUBSTITUTE PARENT {:?}", parent);
    let old_len = self.nodes.len();
    self.nodes.extend(
      substitute
        .into_iter()
        .map(|token| Some(ExpressionBuilder::new(*token, parent))),
    );
    self.stack.extend(old_len..self.nodes.len());
  }
}

pub(super) struct StackNode<'a, 's> {
  pub(super) stack: &'s mut Stack<'a>,
  pub(super) i: usize,
}

impl<'a> StackNode<'a, '_> {
  pub(super) fn builder(&mut self) -> &mut ExpressionBuilder<'a> {
    self.stack.nodes[self.i].as_mut().unwrap()
  }
}
