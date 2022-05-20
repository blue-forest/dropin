/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
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

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

use dropin_helpers::{PortableExpect, PortableUnwrap};

use crate::expressions::Expression;

#[macro_use]
mod error;
pub use error::ParseError;

mod config;
pub use config::Config;
pub mod tokens;
use tokens::Concat;
pub use tokens::Token;

#[derive(Debug)]
pub struct Pattern<'a> {
	key: &'a str,
	token: Box<dyn Token<'a> + 'a>,
}

impl<'a> Pattern<'a> {
	pub fn parse<'b, 'c>(
		&'c self,
		patterns: &'c Patterns<'a, 'b>,
		module: &'b str,
		iter: &mut Peekable<CharIndices<'b>>,
	) -> Result<Expression<'a, 'b>, ParseError<'b>> {
		if let Some((start, _)) = iter.peek() {
			let start = *start;
			let mut result = Expression::new(module.get(start..).punwrap(), self.key);
			self.token.parse(patterns, module, iter, &mut result)?;
			if let Some((end, _)) = iter.peek() {
				result.truncate(*end - start);
			}
			Ok(result)
		} else {
			err!(module, module.len(),
				"unexpected end of file, expected {}",
				self.key.to_string()
			)
		}
	}
}

#[derive(Debug)]
pub struct Patterns<'a, 'b> {
	entry: &'a str,
	patterns: HashMap<&'a str, Pattern<'a>>,
	pub config: Config<'a, 'b>,
}

impl<'a, 'b> Patterns<'a, 'b> {
	pub(crate) fn new(syntax: &'a str) -> Self {
		let mut patterns = HashMap::new();
		let mut iter = syntax.char_indices().peekable();
		let config = Config::new(syntax, &mut iter);
		let mut key = get_key(syntax, &mut iter).pexpect("no pattern found");
		let entry = key;
		loop {
			let token = Concat::parse(syntax, &mut iter);
			if patterns.insert(key, Pattern { key, token }).is_some() {
				panic!("pattern key \"{}\" is used several times", key);
			}
			let key_opt = get_key(syntax, &mut iter);
			if key_opt.is_none() {
				break;
			}
			key = key_opt.punwrap();
		}
		Self {
			entry,
			patterns,
			config,
		}
	}

	pub fn get(&self, key: &str) -> Option<&Pattern<'a>> {
		self.patterns.get(key)
	}

	pub fn parse<'c>(
		&'c self,
		module: &'b str,
	) -> Result<Expression<'a, 'b>, ParseError<'b>> {
		let mut iter = module.char_indices().peekable();
		let result = self.patterns[self.entry].parse(self, module, &mut iter)?;
		if let Some((i, _)) = iter.peek() {
			let remaining = module.get(*i..).punwrap();
			// ignore new line
			if remaining != "\n" {
			  return err!(module, *i,
				  "remaining tokens: \"{}\"", remaining
				);
			}
		}
		Ok(result)
	}
}

fn skip_whitespaces<'a>(iter: &mut Peekable<CharIndices<'a>>) -> Option<usize> {
	for (i, c) in iter {
		if !c.is_whitespace() {
			return Some(i);
		}
	}
	return None;
}

fn get_key<'a>(
	syntax: &'a str,
	iter: &mut Peekable<CharIndices<'a>>,
) -> Option<&'a str> {
	let mut result: Option<&str> = None;
	let start: usize = skip_whitespaces(iter)?;
	for (i, c) in iter {
		if c.is_whitespace() {
			result = Some(syntax.get(start..i).punwrap());
			break;
		}
	}
	result
}
