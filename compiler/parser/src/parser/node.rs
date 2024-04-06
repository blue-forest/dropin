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
