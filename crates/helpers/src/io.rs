#[cfg(target_family = "wasm")]
macro_rules! println {
  ($($arg:tt)*) => ({
  	let message = format_args!($($args)*);
		let data = [
			wasi::Ciovec {
				buf: message.as_ptr(),
				buf_len: message.len(),
			},
			wasi::Ciovec {
				buf: NEW_LINE.as_ptr(),
				buf_len: NEW_LINE.len(),
			},
		];
		unsafe { wasi::fd_write(1, &data).unwrap() };
  })
}

#[cfg(target_family = "wasm")]
macro_rules! panic {
  ($($arg:tt)*) => ({
  	let message = format_args!($($args)*);
		let data = [
			wasi::Ciovec {
				buf: message.as_ptr(),
				buf_len: message.len(),
			},
			wasi::Ciovec {
				buf: NEW_LINE.as_ptr(),
				buf_len: NEW_LINE.len(),
			},
		];
		unsafe { wasi::fd_write(2, &data).unwrap() };
		unsafe { wasi::proc_exit(1) };
  })
}
