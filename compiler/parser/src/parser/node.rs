/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
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

use dropin_compiler_common::token::TokenKind;

pub struct Node<'a> {
  token: TokenKind<'a>,
  children: Vec<Node<'a>>,
  span: Option<(usize, usize)>,
}

impl Node<'_> {
  pub fn print(&self, input: &str, n_indent: usize) {
    let mut indent = String::new();
    for _ in 0..n_indent {
      indent.push_str("  ");
    }
    if let TokenKind::Eof | TokenKind::Empty = self.token {
      return;
    }
    println!("{}{}", indent, self.token.as_str());
    match self.token {
      TokenKind::Terminal(_) => {
        let span = self.span.unwrap();
        println!("{}\"{}\"", indent, &input[span.0..span.1],);
      }
      TokenKind::NonTerminal(_) => {
        for child in &self.children {
          child.print(input, n_indent + 1);
        }
      }
      _ => {}
    }
  }
}

#[derive(Debug)]
pub(super) struct NodeBuilder<'a> {
  pub(super) token: TokenKind<'a>,
  pub(super) children: Vec<usize>,
  pub(super) parent: Option<usize>,
  pub(super) span: Option<(usize, usize)>,
}

impl<'a> NodeBuilder<'a> {
  pub(super) fn new(
    token: TokenKind<'a>,
    parent: Option<usize>,
  ) -> NodeBuilder<'a> {
    NodeBuilder {
      token,
      children: Vec::new(),
      parent,
      span: None,
    }
  }

  pub(super) fn build(
    self,
    nodes: &mut Vec<Option<NodeBuilder<'a>>>,
  ) -> Node<'a> {
    println!(
      "NODES {:?}",
      nodes
        .iter()
        .map(|node| node.as_ref().map(|n| n.token.as_str()))
        .collect::<Vec<_>>()
    );
    println!("{}: {:?}", self.token.as_str(), self.children);
    Node {
      token: self.token,
      children: self
        .children
        .into_iter()
        .map(|i| nodes[i].take().unwrap().build(nodes))
        .collect(),
      span: self.span,
    }
  }
}
