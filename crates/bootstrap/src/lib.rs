use std::str::{self, Utf8Error};

pub mod expressions;
use expressions::Expression;
pub mod modules;
pub mod path;
pub mod syntaxes;
use syntaxes::Patterns;
pub mod utils;

pub struct Recipe<'syntax, 'recipe> {
  syntax:     &'syntax str,
  patterns:   Patterns<'syntax>,
  recipe:     &'recipe str,
  expression: Expression<'syntax, 'recipe>,
}

impl<'syntax, 'recipe> Recipe<'syntax, 'recipe> {
  pub fn new(syntax: &'syntax str, recipe: &'recipe str) -> Self {
    let patterns = Patterns::new(syntax);
    let expression = patterns.parse(recipe).unwrap();
    Self{ syntax, patterns, recipe, expression }
  }
}

const NEW_LINE: &str = "\n";

fn println(message: &str) {
  let stdout = 1;
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
  unsafe { wasi::fd_write(stdout, &data).unwrap() };
}

pub struct Args {
  argv:     Vec<usize>,
  argv_buf: Vec<u8>,
}

impl Args {
  pub unsafe fn new() -> Self {
    let (args_count, args_len) = wasi::args_sizes_get().unwrap();
    let mut argv_buf = vec![0; args_len];
    let argv_buf_ptr = argv_buf.as_mut_ptr();
    let mut argv_ptrs = vec![argv_buf_ptr; args_count];
    wasi::args_get(argv_ptrs.as_mut_ptr(), argv_buf_ptr).unwrap();
    let mut argv = Vec::new();
    for arg in argv_ptrs.iter() {
      argv.push(*arg as usize - argv_buf_ptr as usize);
    }
    Self{ argv, argv_buf }
  }

  pub fn get(&self, i: usize) -> Result<&str, Utf8Error> {
    let start = *self.argv.get(i).unwrap();
    let end = if i + 1 == self.argv.len() {
      self.argv_buf.len()
    } else {
      *self.argv.get(i + 1).unwrap()
    };
    str::from_utf8(self.argv_buf.get(start..end-1).unwrap())
  }
}

#[no_mangle]
pub fn _start() {
  let args = unsafe { Args::new() };
  println(args.get(1).unwrap());
}

