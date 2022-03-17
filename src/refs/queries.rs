#[derive(Debug)]
pub struct Query(String);

impl Query {
  pub fn new(query: String) -> Self {
    Self(query)
  }
}

/* TODO: query creates iterator
impl Iterator for Query {
  type Item = Result<&str, Issue>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.0.is_empty() {
      None
    } else if self.0.starts_with("'") {
      match self.0.get(1..) {
        Some(trimmed) => {
          match trimmed.find("'") {
            Some(close_index) => {
              if close_index+2 < self.0.len() {
                let (key, rest) = self.0.split_at(close_index+2);
                self.0 = rest;
                Some(Ok(key.trim_end_matches(".")))
              } else {
                Some(Ok(""))
              }
            },
            None => Some(Err(Issue{})),
          }
        },
        None => Some(Err(Issue{})),
      }
    } else {
      let (key, rest) = match self.0.split_once(".") {
        Some((key, rest)) => (key, rest),
        None => (self.0, self.0.get(self.0.len()..).unwrap()),
      };
      self.0 = rest;
      Some(Ok(key))
    }
  }
}
*/
