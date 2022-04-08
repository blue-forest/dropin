use std::fs::read_to_string;
use std::path::PathBuf;
use dropin_utils::path::get_root;

fn get_path(collection: &str, id: String) -> PathBuf {
  let mut split = id.split(':');
  let owner = split.next().expect("expected owner");
  let model = split.next().expect("expected model");
  let version = split.next().expect("expected version");
  let mut recipe = split.next().expect("expected recipe").to_string();
  recipe.push_str(".dropin");
  let mut result = get_root();
  result.push(owner);
  // result.push("models");
  result.push(model);
  result.push(version);
  result.push(collection);
  result.push(recipe);
  result
}

pub fn get_recipe(collection: &str, id: String) -> String {
  let path = get_path(collection, id);
  let content = read_to_string(path).unwrap();
  let header_split = content.find("\n===").unwrap();
  let start = content.get(header_split+4..).unwrap().find("\n").unwrap() + header_split + 5;
  content.get(start..).unwrap().to_string()
}

