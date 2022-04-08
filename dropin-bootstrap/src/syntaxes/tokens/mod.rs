use std::fmt::Debug;
use std::iter::Peekable;
use std::str::CharIndices;

use super::{Expression, Patterns, ParseError};

mod concat;
pub use concat::Concat;

mod getter;
pub use getter::Getter;

mod litteral;
pub use litteral::Litteral;

mod quantifier;
pub use quantifier::Quantifier;

pub trait Token<'a>: Debug {
  fn parse<'b, 'c>(
    &self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>,
    expr:     &mut Expression<'a, 'b, 'c>,
  ) -> Result<(), ParseError>;
}

