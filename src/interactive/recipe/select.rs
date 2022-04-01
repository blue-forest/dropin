use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use crate::interactive::{Command, Cli};
use super::Recipe;
use super::remove::Remove;
use super::edit::Edit;

pub struct Select(Arc<Selection>);

impl Select {
  pub fn new(
    recipe:     Arc<dyn Recipe>,
    id:         &str,
    namespaces: Arc<Vec<String>>,
  ) -> Self {
    Self(Arc::new(Selection::new(recipe, id, namespaces)))
  }
}

impl Display for Select {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    format!("recipe {}", self.0.id).fmt(f)
  }
}

impl Command for Select {
  fn run(&self, cli: &mut Cli) -> u32 {
    let commands: Vec<Box<dyn Command>> = vec![
      Box::new(Edit::new(self.0.clone())),
      Box::new(Remove::new(self.0.clone())),
    ];
    let mut id = String::new();
    for namespace in self.0.namespaces.iter() {
      id.push_str(namespace);
      id.push('/');
    }
    id.push_str(&self.0.id);
    cli.run_select(&id, &commands)
  }
}

pub struct Selection {
  namespaces: Arc<Vec<String>>,
  id:         String,
  recipe:     Arc<dyn Recipe>,
}

impl Selection {
  pub fn new(
    recipe:     Arc<dyn Recipe>,
    id:         &str,
    namespaces: Arc<Vec<String>>,
  ) -> Self {
    Self{ recipe, id: id.to_string(), namespaces }
  }

  pub fn namespaces(&self) -> Arc<Vec<String>> { self.namespaces.clone() }
  pub fn id(&self)         -> &str             { &self.id }
  pub fn recipe(&self)     -> Arc<dyn Recipe>  { self.recipe.clone() }
}
