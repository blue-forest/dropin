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

use alloc::string::String;
use dropin_compiler_common::{ir::Expression, token::TokenKind};

use crate::{lexer, Table};

use self::non_terminal::parse_non_terminal;
use self::stack::Stack;
use self::terminal::parse_terminal;

mod ir;
mod non_terminal;
mod stack;
mod terminal;

pub fn parse(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  input: String,
  main_non_terminal: Option<String>,
  table: &Table,
) -> Expression {
  print!(stdout, "{:?}", input);

  let mut tokens = lexer(&input);
  print!(stdout, "{:?}", tokens);

  let mut stack = Stack::new(
    main_non_terminal
      .as_ref()
      .map(|s| s.as_str())
      .unwrap_or("predicate"),
  );

  let mut is_deindent = false;
  let mut current = 0;

  while !stack.is_empty() {
    print!(stdout, "STACK {:?}", stack);

    let mut stack_top = stack.pop();
    let token = stack_top.builder().token;

    let control = match token {
      TokenKind::NonTerminal(name) => {
        let (control, new_is_deindent) = parse_non_terminal(
          #[cfg(debug_assertions)]
          stdout,
          &table,
          &input,
          &mut tokens,
          &mut current,
          stack_top,
          name,
          is_deindent,
        );
        is_deindent = new_is_deindent;
        control
      }
      TokenKind::Empty => LoopControl::Continue,
      TokenKind::Eof => break,
      TokenKind::Deindent => {
        print!(stdout, "DEINDENT");
        is_deindent = true;
        parse_terminal(
          #[cfg(debug_assertions)]
          stdout,
          &tokens,
          &mut current,
          stack_top,
        )
      }
      _ => {
        print!(stdout, "PUSH {}", token.as_str());
        is_deindent = false;
        parse_terminal(
          #[cfg(debug_assertions)]
          stdout,
          &tokens,
          &mut current,
          stack_top,
        )
      }
    };
    if let LoopControl::Break = control {
      break;
    }
  }

  let root = stack.into_expression(
    #[cfg(debug_assertions)]
    stdout,
    &input,
  );

  print!(stdout, "{root:?}");
  root
}

pub enum LoopControl {
  Break,
  Continue,
}
