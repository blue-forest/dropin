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

use crate::token::node_to_token;
use abnf::types::Node;
use dropin_compiler_common::token::TokenKind;

pub struct Concatenation<'a>(Option<Vec<TokenKind<'a>>>);

impl<'a> Concatenation<'a> {
	pub fn new(nodes: &'a [Node]) -> Self {
		Self(Some(nodes.iter().map(|node| node_to_token(node)).collect()))
	}
}

impl<'a> Iterator for Concatenation<'a> {
	type Item = Vec<TokenKind<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take()
	}
}
