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

use crate::{token::node_to_token, Token};

pub struct Term<'a>(Option<Token<'a>>);

impl<'a> Term<'a> {
	pub fn new(node: &'a Node) -> Self {
		Self(Some(node_to_token(node)))
	}
}

impl<'a> Iterator for Term<'a> {
	type Item = Vec<Token<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take().map(|token| vec![token])
	}
}
