const NEW_LINE: &str = "\n";

#[no_mangle]
pub fn print(message: &str) {
  print_to(message, 1)
}

pub fn print_to(message: &str, fd: u32) {
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
  unsafe { wasi::fd_write(fd, &data).unwrap() };
}
