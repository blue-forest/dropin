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
use dropin_compiler_parser_lib::Token;
use indoc::indoc;

#[test]
fn arrays() {
  test_lexer(
    indoc! {"[
			1,
			2,
			3,
		]"},
    vec![
      Token::new(TokenKind::BracSpaced, (0, 1)),
      Token::new(TokenKind::Indent, (2, 3)),
      Token::new(TokenKind::Quantity, (3, 4)),
      Token::new(TokenKind::Comma, (4, 5)),
      Token::new(TokenKind::Newline, (6, 7)),
      Token::new(TokenKind::Quantity, (7, 8)),
      Token::new(TokenKind::Comma, (8, 9)),
      Token::new(TokenKind::Newline, (10, 11)),
      Token::new(TokenKind::Quantity, (11, 12)),
      Token::new(TokenKind::Comma, (12, 13)),
      Token::new(TokenKind::Deindent, (14, 14)),
      Token::new(TokenKind::Rbrac, (14, 15)),
    ],
  );
  test_lexer(
    "a in [1,2]",
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(TokenKind::In, (2, 4)),
      Token::new(TokenKind::BracSpaced, (5, 6)),
      Token::new(TokenKind::Quantity, (6, 7)),
      Token::new(TokenKind::Comma, (7, 8)),
      Token::new(TokenKind::Quantity, (8, 9)),
      Token::new(TokenKind::Rbrac, (9, 10)),
    ],
  );
}

#[test]
fn objects() {
  test_lexer(
    "a = b.c.d",
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(TokenKind::Samekey, (2, 3)),
      Token::new(TokenKind::Id, (4, 5)),
      Token::new(TokenKind::Dot, (5, 6)),
      Token::new(TokenKind::Id, (6, 7)),
      Token::new(TokenKind::Dot, (7, 8)),
      Token::new(TokenKind::Id, (8, 9)),
    ],
  );
  test_lexer(
    indoc! {"{
			a: 1,
			b: \"2\",
		}"},
    vec![
      Token::new(TokenKind::Lbrace, (0, 1)),
      Token::new(TokenKind::Indent, (2, 3)),
      Token::new(TokenKind::Id, (3, 4)),
      Token::new(TokenKind::Block, (4, 5)),
      Token::new(TokenKind::Quantity, (6, 7)),
      Token::new(TokenKind::Comma, (7, 8)),
      Token::new(TokenKind::Newline, (9, 10)),
      Token::new(TokenKind::Id, (10, 11)),
      Token::new(TokenKind::Block, (11, 12)),
      Token::new(TokenKind::Text, (13, 16)),
      Token::new(TokenKind::Comma, (16, 17)),
      Token::new(TokenKind::Deindent, (18, 18)),
      Token::new(TokenKind::Rbrace, (18, 19)),
    ],
  );
}

#[test]
fn parentheses() {
  test_lexer(
    "(a + false) - 1",
    vec![
      Token::new(TokenKind::ParSpaced, (0, 1)),
      Token::new(TokenKind::Id, (1, 2)),
      Token::new(TokenKind::Add, (3, 4)),
      Token::new(TokenKind::False, (5, 10)),
      Token::new(TokenKind::Rpar, (10, 11)),
      Token::new(TokenKind::Sub, (12, 13)),
      Token::new(TokenKind::Quantity, (14, 15)),
    ],
  );
  test_lexer(
    indoc! {"(
			false
			| (
				true
				& false
			)
		)"},
    vec![
      Token::new(TokenKind::ParSpaced, (0, 1)),
      Token::new(TokenKind::Indent, (2, 3)),
      Token::new(TokenKind::False, (3, 8)),
      Token::new(TokenKind::Newline, (9, 10)),
      Token::new(TokenKind::Or, (10, 11)),
      Token::new(TokenKind::ParSpaced, (12, 13)),
      Token::new(TokenKind::Indent, (14, 16)),
      Token::new(TokenKind::True, (16, 20)),
      Token::new(TokenKind::Newline, (21, 23)),
      Token::new(TokenKind::And, (23, 24)),
      Token::new(TokenKind::False, (25, 30)),
      Token::new(TokenKind::Deindent, (31, 32)),
      Token::new(TokenKind::Rpar, (32, 33)),
      Token::new(TokenKind::Deindent, (34, 34)),
      Token::new(TokenKind::Rpar, (34, 35)),
    ],
  );
}
