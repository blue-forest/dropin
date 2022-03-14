use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::Arc;

use pest::iterators::Pair;

use crate::types;
use crate::types::{CustomType, Format, Type};
use super::{read_file, RecipeHeader, Rule};

pub fn read_type(path: PathBuf) -> Arc<dyn Type> {
  let content = read_to_string(path).unwrap();
  let pairs = read_file(content.as_str()).into_inner();
  let (header, content_pair_opt) = RecipeHeader::new(pairs);
  let mut content_pairs = content_pair_opt
      .expect("expected type content")
      .into_inner();
  let template_pair = content_pairs.next().expect("expected type templates");
  let mut type_ = CustomType::new(header.id);
  read_template(&mut type_, template_pair);
  Arc::new(type_)
}

fn read_template(type_: &mut CustomType, pair: Pair<Rule>) {
  if !matches!(pair.as_rule(), Rule::type_templates) {
    panic!("expected type templates, got {:?}", pair.as_rule());
  }
  for template in pair.into_inner() {
    let mut key_value = template.into_inner();
    let key = key_value.next().expect("expected key").as_str();
    let format_pair = key_value.next().expect("expected format");
    let format = read_format(format_pair);
    type_.add_template(key.to_string(), format);
  }
}

pub fn read_format(pair: Pair<Rule>) -> Format {
  let mut pairs = pair.into_inner();
  let type_id = pairs.next().expect("expected format type").as_str();
  let type_ = match type_id.find(':') {
    Some(_index) => { todo!() }
    None => {
      match type_id {
        "text" => types::Text::new(),
        _ => { panic!("unknown type: {}", type_id); }
      }
    }
  };
  Format::new(Arc::new(type_))
}

