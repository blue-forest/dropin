use crate::syntaxes::Pattern;

#[derive(Debug)]
pub struct Expression<'syntax, 'module> {
  #[allow(dead_code)]
  value:    &'module str,
  #[allow(dead_code)]
  pattern:  &'syntax str,
  #[allow(dead_code)]
  children: Vec<Expression<'syntax, 'module>>,
}

impl<'syntax, 'module> Expression<'syntax, 'module> {
  pub fn new(
    value: &'module str,
    pattern:  &Pattern<'syntax>,
  ) -> Self {
    Self{ value, pattern: pattern.key, children: vec![] }
  }

  #[allow(dead_code)]
  fn pattern(&self) -> String { todo!() }
  #[allow(dead_code)]
  fn as_str(&self) -> &'module str { todo!() }
  #[allow(dead_code)]
  fn iter(&self) -> Expressions<'syntax, 'module> { todo!() }

  pub fn add_inner(&mut self, expr: Expression<'syntax, 'module>) {
    self.children.push(expr);
  }

  pub fn truncate(&mut self, i: usize) {
    self.value = self.value.get(..i).unwrap()
  }
}

pub struct Expressions<'syntax, 'module> {
  #[allow(dead_code)]
  parent: &'module Expression<'syntax, 'module>,
}

impl<'syntax, 'module> 
  Iterator for Expressions<'syntax, 'module> {
  type Item = Expression<'syntax, 'module>;
  fn next(&mut self) -> Option<Self::Item> { todo!() }
}

