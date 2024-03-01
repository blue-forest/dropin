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

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
	pub kind: TokenKind<'a>,
	pub span: (usize, usize),
	pub state: Option<TokenState>,
}

impl<'a> Token<'a> {
	pub fn new(kind: TokenKind<'a>, span: (usize, usize)) -> Self {
		Token {
			kind,
			span,
			state: None,
		}
	}

	pub fn new_with_state(
		kind: TokenKind<'a>,
		span: (usize, usize),
		state: Option<TokenState>,
	) -> Self {
		Token { kind, span, state }
	}

	pub fn clone_state(&self, state: TokenState) -> Self {
		Token {
			kind: self.kind,
			span: self.span,
			state: Some(state),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenState {
	is_escaped: Option<bool>,
	decimals: Option<bool>,
}

impl TokenState {
	pub fn new(is_escaped: Option<bool>, decimals: Option<bool>) -> Self {
		TokenState {
			is_escaped,
			decimals,
		}
	}

	pub fn is_escaped(&self) -> bool {
		self.is_escaped.unwrap_or(false)
	}

	pub fn decimals(&self) -> bool {
		self.decimals.unwrap_or(false)
	}
}
