use dropin_compiler_common::token::TokenKind;

use crate::{token::Token, Table};

use super::{stack::StackNode, LoopControl, DEBUG};

pub(super) fn parse_non_terminal(
  table: &Table,
  input: &str,
  tokens: &mut Vec<Token>,
  current: usize,
  stack_top: StackNode,
  name: &str,
  is_deindent: bool,
) -> (LoopControl, bool) {
  let token_type = if current < tokens.len() {
    tokens[current].kind
  } else {
    TokenKind::Eof
  };

  let parent = if !name.ends_with("-") {
    stack_top.stack.push_children(stack_top.i);
    Some(stack_top.i)
  } else {
    None
  };

  let substitute = table
    .data
    .get(&name)
    .map(|non_terminals| non_terminals.get(&token_type))
    .flatten()
    .map(|index| table.productions.get(*index))
    .flatten();

  let substitute = if let Some(substitute) = substitute {
    substitute
  } else {
    if token_type == TokenKind::Eof {
      return (LoopControl::Break, false);
    }

    if is_deindent {
      if DEBUG {
        println!("NEWLINE after DEINDENT");
      }
      tokens.insert(current, Token::new(TokenKind::Newline, (0, 0)));
      if let Some(parent) = parent {
        stack_top.stack.pop_children(parent);
      }
      return (LoopControl::Continue, false);
    }
    panic!("{} unexpected {} {:?}", input, name, token_type);
  };

  if DEBUG {
    println!(
      "Substitution {} + {} => {}",
      name,
      token_type.as_str(),
      substitute
        .iter()
        .map(|sub| sub.as_str())
        .collect::<Vec<_>>()
        .join(", "),
    );
  }

  stack_top.stack.substitute(Some(stack_top.i), substitute);

  (LoopControl::Continue, is_deindent)
}
