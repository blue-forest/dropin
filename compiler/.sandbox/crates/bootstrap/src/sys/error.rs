/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2023 Blue Forest
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

use std::error::Error;

use dropin_core::print_to;

pub trait WasiExpect<T> {
	fn wasi_expect(self, message: &str) -> T;
}

impl<T> WasiExpect<T> for Option<T> {
	fn wasi_expect(self, message: &str) -> T {
		if let Some(result) = self {
			result
		} else {
			print_to(message, 2);
			unsafe { wasi::proc_exit(1) };
			unreachable!()
		}
	}
}

pub trait WasiUnwrap<T> {
	fn wasi_unwrap(self) -> T;
}

impl<T, E: Error> WasiUnwrap<T> for Result<T, E> {
	fn wasi_unwrap(self) -> T {
		match self {
			Ok(result) => result,
			Err(err) => {
				print_to(&format!("{}", err), 2);
				unsafe { wasi::proc_exit(1) };
				unreachable!()
			}
		}
	}
}

impl<T> WasiUnwrap<T> for Option<T> {
	fn wasi_unwrap(self) -> T {
		match self {
			Some(result) => result,
			None => {
				print_to("None unwrapped", 2);
				unsafe { wasi::proc_exit(1) };
				unreachable!()
			}
		}
	}
}
