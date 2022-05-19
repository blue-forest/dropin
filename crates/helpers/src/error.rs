use std::error::Error;

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
