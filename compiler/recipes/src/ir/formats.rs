use super::{Format, FormatInner};

impl Format {
  pub fn new(inner: FormatInner) -> Self {
    Self {
      format_inner: Some(inner),
    }
  }
}
