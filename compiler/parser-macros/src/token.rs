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

use abnf::types::Node;
use dropin_compiler_common::token::TokenKind;

pub fn node_to_token(node: &Node) -> TokenKind {
  match node {
    Node::String(lit) => TokenKind::Terminal(lit.value()),
    Node::Rulename(name) => match name.as_str() {
      "NEWLINE" => TokenKind::Newline,
      "INDENT" => TokenKind::Indent,
      "DEINDENT" => TokenKind::Deindent,
      "PARGLUED" => TokenKind::ParGlued,
      "PARSPACED" => TokenKind::ParSpaced,
      "BRACGLUED" => TokenKind::BracGlued,
      "BRACSPACED" => TokenKind::BracSpaced,
      "IF" => TokenKind::If,
      "ELSE" => TokenKind::Else,
      "TRUE" => TokenKind::True,
      "FALSE" => TokenKind::False,
      "SAMEKEY" => TokenKind::Samekey,
      "ID" => TokenKind::Id,
      "TEXT" => TokenKind::Text,
      "QUANTITY" => TokenKind::Quantity,
      "EMPTY" => TokenKind::Empty,
      "LESSTHAN" => TokenKind::LessThan,
      "MORETHAN" => TokenKind::MoreThan,
      "ATLEAST" => TokenKind::AtLeast,
      "ATMOST" => TokenKind::AtMost,
      _ => TokenKind::NonTerminal(name),
    },
    Node::Repetition { .. } => todo!("Repetition"),
    Node::Group(_) => todo!("Group"),
    Node::Optional(_) => todo!("Optional"),
    Node::TerminalValues(_) => todo!("TerminalValues"),
    Node::Prose(_) => todo!("Prose"),
    _ => unreachable!(),
  }
}
