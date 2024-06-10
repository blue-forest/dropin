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

pub use self::lexer::lexer;
pub use self::model::parse_model;
pub use self::token::Token;

mod child;
mod component;
mod expression;
mod format;
mod keys;
mod lexer;
mod model;
mod text;
mod token;

const EXTENSION: &str = ".dropin.yml";

#[dropin_compiler_recipes_macros::table(
  grammar = "compiler/recipes/src/parser/grammar.abnf"
)]
pub struct Table;
