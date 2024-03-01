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

use crate::production::Production;
use abnf::types::Node;
use dropin_compiler_common::token::TokenKind;

pub struct Alternatives<'a> {
	nodes: &'a [Node],
	current: Option<Production<'a>>,
	next: usize,
}

impl<'a> Alternatives<'a> {
	pub fn new(nodes: &'a [Node]) -> Self {
		Self {
			nodes,
			current: None,
			next: 0,
		}
	}
}

impl<'a> Iterator for Alternatives<'a> {
	type Item = Vec<TokenKind<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let current = if let Some(current) = &mut self.current {
				current
			} else {
				let Some(node) = self.nodes.get(self.next) else {
					return None;
				};
				self.next += 1;
				self.current = Some(Production::new(node));
				self.current.as_mut().unwrap()
			};
			let Some(tokens) = current.next() else {
				self.current = None;
				continue;
			};
			break Some(tokens);
		}
	}
}
