use crate::token::Token;

use super::{stack::StackNode, LoopControl};

pub(super) fn parse_terminal(
  tokens: &[Token],
  current: &mut usize,
  mut stack_top: StackNode,
) -> LoopControl {
  stack_top.builder().span = Some(tokens[*current].span);
  stack_top.stack.push_children(stack_top.i);
  *current += 1;
  LoopControl::Continue
}
