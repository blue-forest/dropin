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

use abnf::types::Rule as ABNFRule;

use crate::production::Production;

pub struct Rules {
	rules: Vec<ABNFRule>,
}

impl Rules {
	pub fn new(rules: Vec<ABNFRule>) -> Self {
		Self { rules }
	}

	pub fn iter(&self) -> Iter {
		Iter::new(self)
	}

	fn get(&self, index: usize) -> Option<&ABNFRule> {
		self.rules.get(index)
	}
}

pub struct Iter<'a> {
	rules: &'a Rules,
	current: usize,
}

impl<'a> Iter<'a> {
	fn new(rules: &'a Rules) -> Self {
		Self { rules, current: 0 }
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = Rule<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let Some(rule) = self.rules.get(self.current) else {
			return None;
		};
		self.current += 1;
		Some(Rule(rule))
	}
}

pub struct Rule<'a>(&'a ABNFRule);

impl<'a> Rule<'a> {
	pub fn name(&self) -> &'a str {
		self.0.name()
	}

	pub fn iter(&self) -> Production<'a> {
		Production::new(self.0.node())
	}
}
