extern crate proc_macro;
use proc_macro2::TokenStream;
use syn::parse_file;

use std::error::Error;
use std::path::Path;

pub const NEW_LINE: &str = "\n";

#[macro_export]
macro_rules! println {
  ($($arg:tt)*) => ({
  	let message = format!($($arg)*);
		let data = [
			wasi::Ciovec {
				buf: message.as_ptr(),
				buf_len: message.len(),
			},
			wasi::Ciovec {
				buf: $crate::NEW_LINE.as_ptr(),
				buf_len: $crate::NEW_LINE.len(),
			},
		];
		unsafe { wasi::fd_write(1, &data).unwrap() };
  })
}

#[macro_export]
macro_rules! panic {
  ($($arg:tt)*) => ({
  	let message = format!($($arg)*);
		let data = [
			wasi::Ciovec {
				buf: message.as_ptr(),
				buf_len: message.len(),
			},
			wasi::Ciovec {
				buf: $crate::NEW_LINE.as_ptr(),
				buf_len: $crate::NEW_LINE.len(),
			},
		];
		unsafe { wasi::fd_write(2, &data).unwrap() };
		unsafe { wasi::proc_exit(1) };
		unreachable!();
  })
}

pub trait PortableExpect<T> {
	fn pexpect(self, message: &str) -> T;
}

impl<T> PortableExpect<T> for Option<T> {
	#[cfg(not(target_family = "wasm"))]
	fn pexpect(self, message: &str) -> T {
		self.expect(message)
	}

	#[cfg(target_family = "wasm")]
	fn pexpect(self, message: &str) -> T {
		if let Some(result) = self {
			result
		} else {
			panic!("{}", message);
		}
	}
}

pub trait PortableUnwrap<T> {
	fn punwrap(self) -> T;
}

impl<T, E: Error> PortableUnwrap<T> for Result<T, E> {
	#[cfg(not(target_family = "wasm"))]
	fn punwrap(self) -> T {
		self.unwrap()
	}

	#[cfg(target_family = "wasm")]
	fn punwrap(self) -> T {
		match self {
			Ok(result) => result,
			Err(err) => {
				panic!("{}", err);
			}
		}
	}
}

impl<T> PortableUnwrap<T> for Option<T> {
	#[cfg(not(target_family = "wasm"))]
	fn punwrap(self) -> T {
		self.unwrap()
	}

	#[cfg(target_family = "wasm")]
	fn punwrap(self) -> T {
		match self {
			Some(result) => result,
			None => {
				panic!("None unwrapped");
			}
		}
	}
}

pub fn read(path: &Path) -> String {
	unsafe {
		let fd = wasi::path_open(
			3, // preopened fd
			wasi::LOOKUPFLAGS_SYMLINK_FOLLOW,
			path.to_str().punwrap(),
			0,
			wasi::RIGHTS_FD_READ,
			wasi::RIGHTS_FD_READ,
			0,
		)
		.punwrap();
		let mut content = vec![];
		loop {
			let mut buf = [0; 2048];
			let size = wasi::fd_read(
				fd,
				&[wasi::Iovec {
					buf: buf.as_mut_ptr(),
					buf_len: buf.len(),
				}],
			)
			.punwrap();
			content.extend(buf.get(..size).punwrap());
			if size < buf.len() || size == 0 {
				break;
			}
		}
		String::from(std::str::from_utf8(&content).punwrap())
	}
}

struct AST {
}

#[dropin]
fn gen(ast: AST) {
	todo!()
}

#[no_mangle]
pub fn _start() {
	let s = read(
		&Path::new("blueforest/models/modules/dropin-core/v1/src/lib.rs"),
	);
  let ast = parse_file(&s).punwrap();
	println!("{:#?}", ast);
}
