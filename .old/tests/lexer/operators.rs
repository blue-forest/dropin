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
use dropin_compiler_common::TokenKind;
use dropin_compiler_parser_lib::Token;

fn test_operator(operator: &str, expected: TokenKind) {
  let shift = operator.len();
  test_lexer(
    format!("a {} b", operator).as_str(),
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(expected, (2, 2 + shift)),
      Token::new(TokenKind::Id, (shift + 3, shift + 4)),
    ],
  );
  test_lexer(
    format!("  a {}b   ", operator).as_str(),
    vec![
      Token::new(TokenKind::Id, (2, 3)),
      Token::new(expected, (4, 4 + shift)),
      Token::new(TokenKind::Id, (shift + 4, shift + 5)),
    ],
  );
  test_lexer(
    format!("a{}b", operator).as_str(),
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(expected, (1, 1 + shift)),
      Token::new(TokenKind::Id, (shift + 1, shift + 2)),
    ],
  );
  test_lexer(
    format!("a      {}         b", operator).as_str(),
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(expected, (7, 7 + shift)),
      Token::new(TokenKind::Id, (shift + 16, shift + 17)),
    ],
  );
  test_lexer(
    format!("aaaaaa{}bbbb", operator).as_str(),
    vec![
      Token::new(TokenKind::Id, (0, 6)),
      Token::new(expected, (6, 6 + shift)),
      Token::new(TokenKind::Id, (shift + 6, shift + 10)),
    ],
  );

  test_values(
    operator,
    expected,
    TokenKind::Quantity,
    &[
      "1", "1.0", "10000",
      // NOT WORKING: 1.0e1, 1.0e+1, 1.0e-1, 1,0
    ],
  );

  test_values(operator, expected, TokenKind::True, &["true"]);
  test_values(operator, expected, TokenKind::False, &["false"]);
  test_values(operator, expected, TokenKind::Text, &["\"test\""]);
}

fn test_values(
  operator: &str,
  expected: TokenKind,
  kind: TokenKind,
  values: &[&str],
) {
  let shift = operator.len();
  for value in values {
    test_lexer(
      format!("value {} {}", operator, value).as_str(),
      vec![
        Token::new(TokenKind::Id, (0, 5)),
        Token::new(expected, (6, 6 + shift)),
        Token::new(kind, (shift + 7, shift + 7 + value.len())),
      ],
    );
  }
}

#[test]
fn equals_to() {
  test_operator("==", TokenKind::EqualsTo);
}

#[test]
fn different_from() {
  test_operator("!=", TokenKind::DifferentFrom);
}

#[test]
fn more_than() {
  test_operator(">", TokenKind::MoreThan);
}

#[test]
fn at_least() {
  test_operator(">=", TokenKind::AtLeast);
}

#[test]
fn less_than() {
  test_operator("<", TokenKind::LessThan);
}

#[test]
fn at_most() {
  test_operator("<=", TokenKind::AtMost);
}

#[test]
fn add() {
  test_operator("+", TokenKind::Add);
}

#[test]
fn subtract() {
  test_operator("-", TokenKind::Sub);
}

#[test]
fn and() {
  test_operator("&", TokenKind::And);
}

#[test]
fn or() {
  test_operator("|", TokenKind::Or);
}

#[test]
fn not() {
  test_lexer(
    "!a",
    vec![
      Token::new(TokenKind::Not, (0, 1)),
      Token::new(TokenKind::Id, (1, 2)),
    ],
  );
  test_lexer(
    "(!a & b) | c",
    vec![
      Token::new(TokenKind::ParSpaced, (0, 1)),
      Token::new(TokenKind::Not, (1, 2)),
      Token::new(TokenKind::Id, (2, 3)),
      Token::new(TokenKind::And, (4, 5)),
      Token::new(TokenKind::Id, (6, 7)),
      Token::new(TokenKind::Rpar, (7, 8)),
      Token::new(TokenKind::Or, (9, 10)),
      Token::new(TokenKind::Id, (11, 12)),
    ],
  );
}

#[test]
fn exists() {
  test_lexer(
    "a?",
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(TokenKind::Exists, (1, 2)),
    ],
  );
}

/*#[test]
fn cast() {
  test_lexer(
    "a = \"\\\"b\\\"",
    vec![
      Token::new(TokenKind::Id, (0, 1)),
      Token::new(TokenKind::Assign, (2, 3)),
    ],
  );
}*/
