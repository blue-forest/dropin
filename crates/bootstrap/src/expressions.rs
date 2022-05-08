/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2022 Blue Forest
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

use std::slice::Iter;

use crate::WasiUnwrap;

#[derive(Debug)]
pub struct Expression<'syntax, 'module> {
    value: &'module str,
    pattern: &'syntax str,
    children: Vec<Expression<'syntax, 'module>>,
}

impl<'syntax, 'module> Expression<'syntax, 'module> {
    pub fn new(value: &'module str, pattern: &'syntax str) -> Self {
        Self {
            value,
            pattern,
            children: vec![],
        }
    }

    pub fn pattern(&self) -> &'syntax str {
        self.pattern
    }

    pub fn as_str(&self) -> &'module str {
        self.value
    }

    pub fn iter(&self) -> Iter<Self> {
        self.children.iter()
    }

    pub fn add_inner(&mut self, expr: Expression<'syntax, 'module>) {
        self.children.push(expr);
    }

    pub fn truncate(&mut self, i: usize) {
        self.value = self.value.get(..i).wasi_unwrap()
    }
}
