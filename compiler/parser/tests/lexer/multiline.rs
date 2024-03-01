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
fn variables() {
	test_lexer(
		indoc! {"
			a = 1
			b = \"ok\"
		"},
		vec![
			Token::new(TokenKind::Id, (0, 1)),
			Token::new(TokenKind::Samekey, (2, 3)),
			Token::new(TokenKind::Quantity, (4, 5)),
			Token::new(TokenKind::Newline, (6, 6)),
			Token::new(TokenKind::Id, (6, 7)),
			Token::new(TokenKind::Samekey, (8, 9)),
			Token::new(TokenKind::Text, (10, 14)),
		],
	);

	test_lexer(
		"	a = 1\n  b = \"ok\"",
		vec![
			Token::new(TokenKind::Id, (1, 2)),
			Token::new(TokenKind::Samekey, (3, 4)),
			Token::new(TokenKind::Quantity, (5, 6)),
			Token::new(TokenKind::Indent, (7, 9)),
			Token::new(TokenKind::Id, (9, 10)),
			Token::new(TokenKind::Samekey, (11, 12)),
			Token::new(TokenKind::Text, (13, 17)),
			Token::new(TokenKind::Deindent, (17, 17)),
		],
	);

	/*test_lexer(
		indoc! {"
			test1 = 1
			test2 = \"ok\"
		"},
		vec![Token::new(TokenKind::Id, (0, 5))],
	);*/
}
