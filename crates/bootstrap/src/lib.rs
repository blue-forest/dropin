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

