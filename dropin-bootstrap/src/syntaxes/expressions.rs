use crate::syntaxes::Pattern;

#[derive(Debug)]
pub struct Expression<'syntax, 'module, 'pattern> {
  #[allow(dead_code)]
  value:    &'module str,
  #[allow(dead_code)]
  pattern:  &'pattern Pattern<'syntax>,
  #[allow(dead_code)]
  children: Vec<Expression<'syntax, 'module, 'pattern>>,
}

impl<'syntax, 'module, 'pattern> Expression<'syntax, 'module, 'pattern> {
  pub fn new(
    value: &'module str,
    pattern:  &'pattern Pattern<'syntax>,
  ) -> Self {
    Self{ value, pattern, children: vec![] }
  }

  #[allow(dead_code)]
  fn pattern(&self) -> String { todo!() }
  #[allow(dead_code)]
  fn as_str(&self) -> &'module str { todo!() }
  #[allow(dead_code)]
  fn iter(&self) -> Expressions<'syntax, 'module, 'pattern> { todo!() }
  pub fn add_inner(&mut self, expr: Expression<'syntax, 'module, 'pattern>) {
    self.children.push(expr);
  }
}

pub struct Expressions<'syntax, 'module, 'pattern> {
  #[allow(dead_code)]
  parent: &'module Expression<'syntax, 'module, 'pattern>,
}

impl<'syntax, 'module, 'pattern> 
  Iterator for Expressions<'syntax, 'module, 'pattern> {
  type Item = Expression<'syntax, 'module, 'pattern>;
  fn next(&mut self) -> Option<Self::Item> { todo!() }
}

