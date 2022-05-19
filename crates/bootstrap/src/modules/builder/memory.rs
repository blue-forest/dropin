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

use super::ModuleBuilder;
use wasm_encoder::{DataCountSection, DataSection};

#[derive(Default)]
pub struct MemoryBuilder<'a> {
	data: Vec<&'a [u8]>,
	data_len: usize,
}

impl<'module> ModuleBuilder<'module> {
	pub fn memory(&mut self) -> &mut MemoryBuilder<'module> {
		&mut self.memory
	}
}

impl<'a> MemoryBuilder<'a> {
	pub fn passive(&mut self, data: &'a [u8]) -> usize {
		let result = self.data.len();
		self.data_len += data.len();
		self.data.push(data);
		result
	}

	pub(super) fn build_data_count(&self) -> Option<DataCountSection> {
		if self.data.is_empty() {
			return None;
		}
		Some(DataCountSection {
			count: self.data.len() as u32,
		})
	}

	pub(super) fn build_data(&self) -> Option<DataSection> {
		if self.data.is_empty() {
			return None;
		}
		let mut result = DataSection::new();
		for d in self.data.iter() {
			result.passive(d.iter().copied());
		}
		Some(result)
	}
}
