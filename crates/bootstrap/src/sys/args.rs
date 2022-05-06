use std::str::{self, Utf8Error};

use super::WasiUnwrap;

pub struct Args {
  argv:     Vec<usize>,
  argv_buf: Vec<u8>,
}

impl Args {
  pub unsafe fn new() -> Self {
    let (args_count, args_len) = wasi::args_sizes_get().wasi_unwrap();
    let mut argv_buf = vec![0; args_len];
    let argv_buf_ptr = argv_buf.as_mut_ptr();
    let mut argv_ptrs = vec![argv_buf_ptr; args_count];
    wasi::args_get(argv_ptrs.as_mut_ptr(), argv_buf_ptr).wasi_unwrap();
    let mut argv = Vec::new();
    for arg in argv_ptrs.iter() {
      argv.push(*arg as usize - argv_buf_ptr as usize);
    }
    Self{ argv, argv_buf }
  }

  pub fn get(&self, i: usize) -> Result<&str, Utf8Error> {
    let start = *self.argv.get(i).wasi_unwrap();
    let end = if i + 1 == self.argv.len() {
      self.argv_buf.len()
    } else {
      *self.argv.get(i + 1).wasi_unwrap()
    };
    str::from_utf8(self.argv_buf.get(start..end-1).wasi_unwrap())
  }
  
  pub fn len(&self) -> usize { self.argv.len() }
}

