use std::{collections::BTreeMap, fs::File, io::Read, path::Path};

use anyhow::Result;
use dropin_compiler_common::to_upper_camelcase;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::{
  ir::{Component, Model, Page},
  parser::EXTENSION,
};

pub fn parse_model(root: &Path) -> Result<Model> {
  let mut f = File::open(root.join("model.dropin.yml"))?;
  let mut recipe = String::new();
  f.read_to_string(&mut recipe)?;
  let model = serde_yaml::from_str::<ModelRecipe>(&recipe)?;
  let mut pages = BTreeMap::new();
  let mut components = Vec::new();
  let mut component_to_page = model
    .app
    .pages
    .iter()
    .map(|(key, page)| (page.component.as_str(), key))
    .collect::<BTreeMap<_, _>>();

  let components_path = root.join("components");
  let components_path_len = components_path.to_str().unwrap().len();
  for entry in WalkDir::new(&components_path) {
    let entry = entry?;
    if entry.metadata()?.is_dir() {
      continue;
    }
    let path = entry.path();
    println!("{}", path.display());
    let mut f = File::open(path)?;
    let mut recipe = String::new();
    f.read_to_string(&mut recipe)?;
    let mut component = serde_yaml::from_str::<Component>(&recipe)?;
    let path_str = path.to_str().unwrap();
    let id =
      &path_str[components_path_len + 1..path_str.len() - EXTENSION.len()];
    component.set_name(to_upper_camelcase(id));
    if let Some(key) = component_to_page.remove(id) {
      let page = model.app.pages.get(key).unwrap();
      let index = components.len() as u64;
      pages.insert(
        key.clone(),
        Page {
          component: index,
          title: page.title.clone(),
          path: page.path.clone(),
        },
      );
    }
    components.push(component);
  }

  Ok(Model {
    name: model.app.name,
    version: model.app.version,
    pages,
    components,
  })
}

#[derive(Deserialize)]
struct ModelRecipe {
  app: ModelApp,
}

#[derive(Deserialize)]
struct ModelApp {
  name: String,
  version: String,
  pages: BTreeMap<String, PageRecipe>,
}

#[derive(Deserialize)]
struct PageRecipe {
  component: String,
  title: String,
  path: String,
}
