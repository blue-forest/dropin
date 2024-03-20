use crate::token::Token;

use super::{node::NodeBuilder, stack::Stack, LoopControl};

pub(super) fn parse_terminal<'a>(
	tokens: &[Token],
	current: &mut usize,
	stack: &'a mut Stack,
	stack_top_index: usize,
	stack_top: &'a mut NodeBuilder,
) -> LoopControl {
	stack_top.span = Some(tokens[*current].span);
	stack.push_children(stack_top_index);
	*current += 1;
	LoopControl::Continue
}
