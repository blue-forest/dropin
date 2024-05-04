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

use core::fmt::{self, Debug, Formatter};

use super::{Arithmetic, Comparison, Control, Logic, Value};

#[derive(Clone)]
pub enum Expression {
  Value(Value),
  Arithmetic(Arithmetic),
  Comparison(Comparison),
  Logic(Logic),
  Control(Control),
}

impl Debug for Expression {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Expression::Value(inner) => inner.fmt(f),
      Expression::Arithmetic(inner) => inner.fmt(f),
      Expression::Comparison(inner) => inner.fmt(f),
      Expression::Logic(inner) => inner.fmt(f),
      Expression::Control(inner) => inner.fmt(f),
    }
  }
}
