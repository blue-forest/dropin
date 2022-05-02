use std::str::{self, Utf8Error};

use dropin_modules::{print, print_to};

pub mod path;
use path::{get_model, get_recipe};
pub mod expressions;
use expressions::Expression;
pub mod modules;
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
    let expression = patterns.parse(recipe).wasi_unwrap();
    Self{ syntax, patterns, recipe, expression }
  }
}

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

trait WasiExpect<T> {
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

trait WasiUnwrap<T> {
  fn wasi_unwrap(self) -> T;
}

use std::error::Error;
impl<T, E: Error> WasiUnwrap<T> for Result<T, E> {
  fn wasi_unwrap(self) -> T {
    match self {
      Ok(result) => result,
      Err(err) => {
        print_to(&format!("{}", err), 2);
        // unsafe { wasi::proc_exit(1) };
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

const SYNTAX_MODELS: &str = "blueforest:dropin-modules:v1:Models";

#[no_mangle]
pub fn _start() {
  let args = unsafe { Args::new() };
  if args.len() != 2 {
    print_to("expected argument: <model>", 2);
    unsafe { wasi::proc_exit(1) };
  }
  let syntax_content = &get_recipe("syntaxes", SYNTAX_MODELS);
  let model_content = &get_model(args.get(1).wasi_unwrap());
  let recipe = Recipe::new(syntax_content, model_content);
  print(&format!("{:?}", recipe.patterns));
  print(&format!("{:?}", recipe.expression));
}

