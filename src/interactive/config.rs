use serde_derive::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use std::fs::{read_to_string, write};

pub struct Config {
  path:    PathBuf,
  content: Content,
}

#[derive(Deserialize, Serialize)]
struct Content {
  owner: Option<String>,
  model: Option<String>,
}

impl Config {
  pub fn new(root: &Path) -> Self {
    let mut path = root.to_path_buf();
    path.push("config.toml");
    let content = if !path.exists() {
      Content{ owner: None, model: None }
    } else {
      let file_content = read_to_string(&path).unwrap();
      toml::from_str(&file_content).unwrap()
    };
    Self{ path, content }
  }

  pub fn set_owner(&mut self, owner: String) {
    self.content.owner = Some(owner);
    self.save();
  }

  pub fn owner(&self) -> &Option<String> { &self.content.owner }

  pub fn set_model(&mut self, model: String) {
    self.content.model = Some(model);
    self.save();
  }

  pub fn model(&self) -> &Option<String> { &self.content.model }

  fn save(&self) {
    write(&self.path, toml::to_string(&self.content).unwrap()).unwrap();
  }
}
