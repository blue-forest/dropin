use std::error::Error;

use dropin_modules::print_to;


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

