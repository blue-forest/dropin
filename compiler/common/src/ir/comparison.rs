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

use super::Expression;

#[derive(Debug)]
pub enum Comparison {
  EqualsTo(Box<Expression>, Box<Expression>),
  DifferentFrom(Box<Expression>, Box<Expression>),
  In(Box<Expression>, Box<Expression>),
  LessThan(Box<Expression>, Box<Expression>),
  MoreThan(Box<Expression>, Box<Expression>),
  AtLeast(Box<Expression>, Box<Expression>),
  AtMost(Box<Expression>, Box<Expression>),
}
