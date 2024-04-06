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

use crate::{lexer::lexer, Table};
use dropin_compiler_common::token::TokenKind;

pub use self::node::Node;
use self::{
  non_terminal::parse_non_terminal, stack::Stack, terminal::parse_terminal,
};

mod node;
mod non_terminal;
mod stack;
mod terminal;

const DEBUG: bool = true;

pub fn parse<'a>(
  input: &str,
  main_non_terminal: Option<&'a str>,
  table: &'a Table,
) -> Node<'a> {
  if DEBUG {
    println!("{:?}", input);
  }

  let mut tokens = lexer(input);
  if DEBUG {
    println!("{:?}", tokens);
  }

  let mut stack = Stack::new(main_non_terminal);

  let mut is_deindent = false;
  let mut current = 0;

  while !stack.is_empty() {
    if DEBUG {
      println!("STACK {:?}", stack);
    }

    let (stack_top_index, stack_top) = stack.pop();

    let control = match stack_top.token {
      TokenKind::NonTerminal(name) => parse_non_terminal(
        &table,
        &input,
        &mut tokens,
        current,
        &mut stack,
        stack_top_index,
        &name,
        is_deindent,
      ),
      TokenKind::Empty => LoopControl::Continue,
      TokenKind::Eof => break,
      TokenKind::Deindent => {
        if DEBUG {
          println!("DEINDENT");
        }
        is_deindent = true;
        parse_terminal(
          &tokens,
          &mut current,
          &mut stack,
          stack_top_index,
          stack_top,
        )
      }
      _ => {
        if DEBUG {
          println!("PUSH {}", stack_top.token.as_str());
        }
        is_deindent = false;
        parse_terminal(
          &tokens,
          &mut current,
          &mut stack,
          stack_top_index,
          stack_top,
        )
      }
    };
    if let LoopControl::Break = control {
      break;
    }
  }

  let root = stack.into_tree();

  if DEBUG {
    root.print(input, 0);
  }

  root
}

pub enum LoopControl {
  Break,
  Continue,
}
