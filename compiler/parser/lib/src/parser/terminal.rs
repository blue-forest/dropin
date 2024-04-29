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

#[cfg(debug_assertions)]
use core::fmt::Write;

use crate::token::Token;

use super::{stack::StackNode, LoopControl};

pub(super) fn parse_terminal(
  #[cfg(debug_assertions)] stdout: &mut impl Write,
  tokens: &[Token],
  current: &mut usize,
  mut stack_top: StackNode,
) -> LoopControl {
  stack_top.builder().span = Some(tokens[*current].span);
  stack_top.stack.push_children(
    #[cfg(debug_assertions)]
    stdout,
    stack_top.i,
  );
  *current += 1;
  LoopControl::Continue
}
