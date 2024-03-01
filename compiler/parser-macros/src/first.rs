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

use dropin_compiler_common::token::TokenKind;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct First<'a>(HashMap<TokenKind<'a>, HashSet<TokenKind<'a>>>);

impl<'a> First<'a> {
	pub fn insert_non_terminal(&mut self, name: &'a str) {
		self.0.insert(TokenKind::NonTerminal(name), HashSet::new());
	}

	pub fn insert_terminal(&mut self, token: TokenKind<'a>) {
		self.0.insert(token, HashSet::from([token]));
	}

	pub fn init(&mut self, productions: &[(&'a str, Vec<TokenKind<'a>>)]) {
		let mut has_changed = true;
		while has_changed {
			has_changed = false;
			for (name, tokens) in productions.iter() {
				let mut iter = tokens.iter();
				let mut token = iter.next().unwrap();
				let mut right_hand: HashSet<TokenKind> =
					HashSet::from_iter(self.iter_filter_empty(token));
				let mut is_ended = false;
				loop {
					if !self.contains_empty(token) {
						break;
					}
					let next = iter.next();
					if let Some(t) = next {
						token = t;
					} else {
						is_ended = true;
						break;
					};
					right_hand.extend(self.iter_filter_empty(token));
				}
				if is_ended && self.contains_empty(token) {
					right_hand.insert(TokenKind::Empty);
				}
				has_changed =
					has_changed || self.extend(&TokenKind::NonTerminal(name), right_hand);
			}
		}
	}

	pub fn get(&self, token: &TokenKind<'a>) -> &HashSet<TokenKind<'a>> {
		self.0.get(token).unwrap()
	}

	pub fn iter<'b>(
		&'b self,
		token: &TokenKind<'a>,
	) -> impl Iterator<Item = TokenKind<'a>> + 'b {
		self.0.get(token).unwrap().iter().map(|current| *current)
	}

	pub fn iter_filter_empty<'b>(
		&'b self,
		token: &TokenKind<'a>,
	) -> impl Iterator<Item = TokenKind<'a>> + 'b {
		let Some(first) = self.0.get(token) else {
			panic!("{token:?} not found");
		};
		first.iter().filter_map(|current| {
			if let TokenKind::Empty = current {
				None
			} else {
				Some(*current)
			}
		})
	}

	/*
	pub fn into_iter_non_terminals(
		self,
	) -> impl Iterator<Item = (&'a str, HashSet<Token<'a>>)> {
		self.0.into_iter().filter_map(|(key, tokens)| {
			if let Token::NonTerminal(name) = key {
				Some((name, tokens))
			} else {
				None
			}
		})
	}
	*/

	pub fn contains_empty(&self, token: &TokenKind<'a>) -> bool {
		self.0.get(token).unwrap().contains(&TokenKind::Empty)
	}

	pub fn extend(
		&mut self,
		token: &TokenKind<'a>,
		tokens: HashSet<TokenKind<'a>>,
	) -> bool {
		let first = self.0.get_mut(token).unwrap();
		let old_len = first.len();
		first.extend(tokens);
		old_len != first.len()
	}
}
