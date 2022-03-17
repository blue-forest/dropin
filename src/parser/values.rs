use pest::iterators::Pairs;

use crate::refs::{Byte, Ref};
use super::Rule;

pub fn read_value(mut pairs: Pairs<Rule>) -> Box<dyn Ref> {
  let pair = pairs.next().expect("expected value");
  let ref_ = match pair.as_rule() {
    Rule::binary => read_binary(pair.into_inner()),
    _ => { panic!("unknown value: {}", pair.as_str()); }
  };
  Box::new(ref_)
}

pub fn read_binary(mut pairs: Pairs<Rule>) -> Byte {
  let pair = pairs.next().expect("expected binary value");
  match pair.as_rule() {
    Rule::bits => { todo!() }
    Rule::hexa => {
      let hexa = pair.as_str().as_bytes();
      if hexa.len() > 2 {
        panic!("hexadecimal value overflow");
      }
      let mut byte = hexa_unit(*hexa.get(0).expect("expected hexa unit"));
      if let Some(unit) = hexa.get(1) {
        byte = byte * 16 + hexa_unit(*unit);
      }
      Byte::new(byte)
    }
    _ => { panic!("unknown binary value: {}", pair.as_str()); }
  }
}

pub fn hexa_unit(unit: u8) -> u8 {
  let result = if unit >= b'a' {
    unit - b'a' + 10
  } else if unit >= b'A' {
    unit - b'A' + 10
  } else if unit >= b'0' {
    unit - b'0'
  } else {
    panic!("unknown hexa unit: {}", char::from(unit));
  };
  if result > 15 {
    panic!("unknown hexa unit: {}", char::from(unit));
  }
  result
}
