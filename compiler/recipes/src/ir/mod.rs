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

include!(concat!(env!("OUT_DIR"), "/dropin.components.rs"));

mod arithmetics;
mod comparisons;
mod components;
mod controls;
mod formats;
mod keys;
mod logics;
mod text;
mod values;

use arithmetic::ArithmeticInner;
use comparison::ComparisonInner;
use control::ControlInner;
use expression::ExpressionInner;
pub use format::FormatInner;
use logic::LogicInner;
use rich_text_part::RichTextInner;
use value::ValueInner;
