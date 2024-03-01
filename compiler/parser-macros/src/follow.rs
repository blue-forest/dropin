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

use crate::first::First;
use dropin_compiler_common::token::TokenKind;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct Follow<'a>(HashMap<&'a str, HashSet<TokenKind<'a>>>);

impl<'a> Follow<'a> {
	pub fn insert_non_terminal(&mut self, name: &'a str, is_first: bool) {
		let follow = self.0.entry(name).or_insert(HashSet::new());
		if is_first {
			follow.insert(TokenKind::Eof);
		}
	}

	pub fn init(
		&mut self,
		first: &First<'a>,
		productions: &[(&'a str, Vec<TokenKind<'a>>)],
	) {
		let mut has_changed = true;
		while has_changed {
			has_changed = false;
			for (name, tokens) in productions.iter() {
				let mut trailer = self.0.get(name).unwrap().clone();
				for token in tokens.iter().rev() {
					if let TokenKind::NonTerminal(token_name) = token {
						let Some(follow) = self.0.get_mut(token_name) else {
							panic!("{token_name} not found");
						};
						let old_len = follow.len();
						follow.extend(trailer.iter());
						has_changed = has_changed || old_len != follow.len();
						if first.contains_empty(token) {
							trailer.extend(first.iter_filter_empty(token));
							continue;
						}
					}
					trailer = HashSet::from_iter(first.iter(token));
				}
			}
		}
	}

	pub fn get(&self, name: &'a str) -> &HashSet<TokenKind<'a>> {
		self.0.get(name).unwrap()
	}
}
