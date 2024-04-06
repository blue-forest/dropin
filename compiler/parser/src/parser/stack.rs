use std::fmt::{Debug, Formatter};

use dropin_compiler_common::token::TokenKind;

use super::{node::NodeBuilder, Node, DEBUG};

pub(super) struct Stack<'a> {
  nodes: Vec<Option<NodeBuilder<'a>>>,
  stack: Vec<usize>,
}

impl Debug for Stack<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut first = true;
    write!(f, "ALL NODES\n")?;
    for node in &self.nodes {
      if !first {
        write!(f, ", ")?;
      }
      write!(f, "{:?}", node.as_ref().unwrap().token.as_str())?;
      first = false;
    }
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
  pub(super) fn new(main_non_terminal: Option<&'a str>) -> Stack<'a> {
    Stack {
      nodes: vec![
        Some(NodeBuilder::new(TokenKind::NonTerminal("root"), None)),
        Some(NodeBuilder::new(
          TokenKind::NonTerminal(main_non_terminal.unwrap_or("predicate")),
          Some(0),
        )),
      ],
      stack: vec![0, 1],
    }
  }

  pub(super) fn push(&mut self, node: NodeBuilder<'a>) {
    self.nodes.push(node.into());
    self.stack.push(self.nodes.len() - 1);
  }

  pub(super) fn pop(&mut self) -> (usize, &mut NodeBuilder<'a>) {
    let i = self.stack.pop().unwrap();
    (i, self.nodes[i].as_mut().unwrap())
  }

  pub(super) fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  pub(super) fn into_tree(mut self) -> Node<'a> {
    let i = self.nodes[0].as_mut().unwrap().children.pop().unwrap();
    self.nodes[i].take().unwrap().build(&mut self.nodes)
  }

  pub(super) fn push_children(&mut self, i: usize) {
    let node = self.nodes[i].as_ref().unwrap();
    let parent = node.parent.unwrap();
    if DEBUG {
      println!("PUSH_CHILDREN {:?} {:?}", node.token.as_str(), parent);
    }
    self.nodes[parent].as_mut().unwrap().children.push(i);
  }

  pub(super) fn pop_children(&mut self, parent: usize) {
    self.nodes[parent].as_mut().unwrap().children.pop();
  }

  pub(super) fn substitute(
    &mut self,
    mut parent: Option<usize>,
    substitute: &[TokenKind<'a>],
  ) {
    while let Some(new_parent) = parent {
      // if DEBUG {
      // 	println!("NEW_PARENT {:?}", new_parent);
      // }
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
    if DEBUG {
      println!("SUBSTITUTE PARENT {:?}", parent);
    }
    let old_len = self.nodes.len();
    self.nodes.extend(
      substitute
        .into_iter()
        .map(|token| Some(NodeBuilder::new(*token, parent))),
    );
    self.stack.extend(old_len..self.nodes.len());
  }
}
