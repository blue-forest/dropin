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

use crate::{
  alternatives::Alternatives, concatenation::Concatenation, term::Term,
};
use abnf::types::Node;
use dropin_compiler_common::TokenKind;

pub struct Production<'a>(Box<dyn Iterator<Item = Vec<TokenKind<'a>>> + 'a>);

impl<'a> Production<'a> {
  pub fn new(node: &'a Node) -> Self {
    Self(match node {
      Node::Alternatives(nodes) => Box::new(Alternatives::new(nodes)),
      Node::Concatenation(nodes) => Box::new(Concatenation::new(nodes)),
      Node::Repetition { .. } => todo!("Repetition"),
      _ => Box::new(Term::new(node)),
    })
  }
}

impl<'a> Iterator for Production<'a> {
  type Item = Vec<TokenKind<'a>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}
