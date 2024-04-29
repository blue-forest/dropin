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
use dropin_compiler_common::token::TokenKind;

use crate::{token::Token, Table};

use super::{stack::StackNode, LoopControl};

pub(super) fn parse_non_terminal(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
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
    stack_top.stack.push_children(
      #[cfg(debug_assertions)]
      stdout,
      stack_top.i,
    );
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
      print!(stdout, "NEWLINE after DEINDENT");
      tokens.insert(current, Token::new(TokenKind::Newline, (0, 0)));
      if let Some(parent) = parent {
        stack_top.stack.pop_children(parent);
      }
      return (LoopControl::Continue, false);
    }
    panic!("{} unexpected {} {:?}", input, name, token_type);
  };

  print!(
    stdout,
    "Substitution {} + {} => {}",
    name,
    token_type.as_str(),
    substitute
      .iter()
      .map(|sub| sub.as_str())
      .collect::<Vec<_>>()
      .join(", "),
  );

  stack_top.stack.substitute(
    #[cfg(debug_assertions)]
    stdout,
    Some(stack_top.i),
    substitute,
  );

  (LoopControl::Continue, is_deindent)
}
