use std::collections::BTreeMap;
use std::vec::Vec;

use dropin_compiler_common::TokenKind;

use crate::ir::Expression;
use crate::parser::expression::ir::{BuildState, ExpressionBuilder};

pub(super) fn build(
  children: &[usize],
  nodes: &mut Vec<Option<ExpressionBuilder>>,
  input: &str,
  mut state: BuildState,
) -> Expression {
  let mut values = BTreeMap::new();

  let mut key = state.value_indent_id.take().unwrap();
  let mut child_offset = 1;
  let mut children = children.to_vec();
  loop {
    let child_index = 1 - child_offset;
    child_offset = 0;

    let value_node = nodes[children[child_index]].take().unwrap();
    let value_node = nodes[value_node.children[0]].take().unwrap();
    let value_token = &value_node.token;
    let value = match value_token {
      TokenKind::NonTerminal("value") => {
        let state = state.clone();
        value_node.build_inner(nodes, input, state)
      }
      TokenKind::Samekey => Expression::getter(key.into(), Vec::new()),
      _ => unreachable!("{value_token:?}"),
    };
    values.insert(key.into(), value);

    if children.len() == child_index + 1 {
      break;
    }

    children = nodes[children[child_index + 2]]
      .as_ref()
      .unwrap()
      .children
      .clone();

    let (start, end) =
      nodes[children[0]].as_ref().unwrap().span.clone().unwrap();
    key = &input[start..end];
  }

  Expression::object(values)
}
