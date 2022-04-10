use std::slice::Iter;

#[derive(Debug)]
pub struct Expression<'syntax, 'module> {
  value:    &'module str,
  pattern:  &'syntax str,
  children: Vec<Expression<'syntax, 'module>>,
}

impl<'syntax, 'module> Expression<'syntax, 'module> {
  pub fn new(
    value:   &'module str,
    pattern: &'syntax str,
  ) -> Self {
    Self{ value, pattern, children: vec![] }
  }

  pub fn pattern(&self) -> &'syntax str { self.pattern }

  pub fn as_str(&self) -> &'module str { self.value }

  pub fn iter<'expr>(&'expr self) -> Iter<Self> {
    self.children.iter()
  }

  pub fn add_inner(&mut self, expr: Expression<'syntax, 'module>) {
    self.children.push(expr);
  }

  pub fn truncate(&mut self, i: usize) {
    self.value = self.value.get(..i).unwrap()
  }
}
