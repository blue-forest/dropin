pub const NEW_LINE: &str = "\n";

#[cfg(target_family = "wasm")]
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
				buf: $crate::io::NEW_LINE.as_ptr(),
				buf_len: $crate::io::NEW_LINE.len(),
			},
		];
		unsafe { wasi::fd_write(1, &data).unwrap() };
  })
}

#[cfg(target_family = "wasm")]
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
				buf: $crate::io::NEW_LINE.as_ptr(),
				buf_len: $crate::io::NEW_LINE.len(),
			},
		];
		unsafe { wasi::fd_write(2, &data).unwrap() };
		unreachable!();
		unsafe { wasi::proc_exit(1) };
  })
}
