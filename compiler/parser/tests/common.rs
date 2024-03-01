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

use dropin_compiler_parser::{lexer::lexer, token::Token};

pub fn test_lexer(input: &str, expected: Vec<Token>) {
	println!("> `{}`", input.replace("\n", "`  `"));
	let result = lexer(input);
	if result != expected {
		panic!(
			"/!\\ Test failed /!\\\n\n# Input\n`{}`\n\n# Expected\n{}\n# Got\n{}",
			input,
			expected
				.iter()
				.map(|t| format!("- {} -> {} : {:?}\n", t.span.0, t.span.1, t.kind))
				.collect::<Vec<String>>()
				.join(""),
			result
				.iter()
				.map(|t| format!("- {} -> {} : {:?}\n", t.span.0, t.span.1, t.kind))
				.collect::<Vec<String>>()
				.join(""),
		);
	}
}
