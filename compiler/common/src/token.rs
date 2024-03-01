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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TokenKind<'a> {
	Terminal(&'a str),
	NonTerminal(&'a str),
	Newline,
	Indent,
	Deindent,
	ParGlued,
	ParSpaced,
	BracGlued,
	BracSpaced,
	If,
	Else,
	True,
	False,
	Samekey,
	Id,
	Text,
	Quantity,
	LessThan,
	MoreThan,
	AtLeast,
	AtMost,
	Empty,
	End,
	Eof,
	Block,
	EqualsTo,
	DifferentFrom,
	In,
	Add,
	Sub,
	Dot,
	Comma,
	And,
	Or,
	Not,
	Rpar,
	Rbrac,
	Lbrace,
	Rbrace,
	Exists,
	Backslash,
}

impl<'a> TokenKind<'a> {
	pub fn as_str(&self) -> &'a str {
		match self {
			Self::Terminal(name) => name,
			Self::NonTerminal(name) => name,
			Self::Newline => "NEWLINE",
			Self::Indent => "INDENT",
			Self::Deindent => "DEINDENT",
			Self::ParGlued => "PARGLUED",
			Self::ParSpaced => "PARSPACED",
			Self::BracGlued => "BRACGLUED",
			Self::BracSpaced => "BRACSPACED",
			Self::If => "IF",
			Self::Else => "ELSE",
			Self::True => "TRUE",
			Self::False => "FALSE",
			Self::Samekey => "SAMEKEY",
			Self::Id => "ID",
			Self::Text => "TEXT",
			Self::Quantity => "QUANTITY",
			Self::LessThan => "LESSTHAN",
			Self::MoreThan => "MORETHAN",
			Self::AtLeast => "ATLEAST",
			Self::AtMost => "ATMOST",
			Self::Empty => "EMPTY",
			Self::End => "END",
			Self::Eof => "EOF",

			Self::Block => ":",
			Self::EqualsTo => "==",
			Self::DifferentFrom => "!=",
			Self::In => "in",
			Self::Add => "+",
			Self::Sub => "-",
			Self::Dot => ".",
			Self::Comma => ",",
			Self::And => "&",
			Self::Or => "|",
			Self::Not => "!",
			Self::Rpar => ")",
			Self::Rbrac => "]",
			Self::Lbrace => "{",
			Self::Rbrace => "}",
			Self::Exists => "?",
			Self::Backslash => "\\",
		}
	}
}
