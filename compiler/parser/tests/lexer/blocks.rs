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

use crate::common::test_lexer;
use dropin_compiler_common::token::TokenKind;
use dropin_compiler_parser::token::Token;
use indoc::indoc;

#[test]
fn conditions() {
	test_lexer(
		indoc! {"if a == b {
			1
		} else {
			2
		}"},
		vec![
			Token::new(TokenKind::If, (0, 2)),
			Token::new(TokenKind::Id, (3, 4)),
			Token::new(TokenKind::EqualsTo, (5, 7)),
			Token::new(TokenKind::Id, (8, 9)),
			Token::new(TokenKind::Lbrace, (10, 11)),
			Token::new(TokenKind::Indent, (12, 13)),
			Token::new(TokenKind::Quantity, (13, 14)),
			Token::new(TokenKind::Deindent, (15, 15)),
			Token::new(TokenKind::Rbrace, (15, 16)),
			Token::new(TokenKind::Else, (17, 21)),
			Token::new(TokenKind::Lbrace, (22, 23)),
			Token::new(TokenKind::Indent, (24, 25)),
			Token::new(TokenKind::Quantity, (25, 26)),
			Token::new(TokenKind::Deindent, (27, 27)),
			Token::new(TokenKind::Rbrace, (27, 28)),
		],
	);
}
