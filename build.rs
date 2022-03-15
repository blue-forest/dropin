use std::fs::{create_dir, read_to_string, write};
use std::ops::Index;

fn main() {
  let mut main = read_to_string("parser/types.pest").unwrap();
  replace_dynamics(&mut main);
  let _ = create_dir("target/parser");
  // println!("{}", main);
  // std::process::exit(1);
  write("target/parser/types.pest", main).unwrap();
}

fn replace_dynamics(content: &mut String) {
  let dynamics = get_dynamics(content);
  let mut offset = 0;
  for (start, end) in dynamics.iter() {
    let mut path = "parser/".to_string();
    path.push_str(content.index((*start+offset)..(*end+offset)));
    path.push_str(".pest");
    let mut sub_content = read_to_string(path).unwrap();
    replace_dynamics(&mut sub_content);
    content.replace_range((*start-3+offset)..(*end+3+offset), &sub_content);
    offset += sub_content.len() - (end - start + 6);
  }
}

fn get_dynamics(content: &String) -> Vec<(usize, usize)> {
  let mut iter = content.chars();
  let mut in_dynamic = false;
  let mut i = 0;
  let mut start = 0;
  let mut result = Vec::new();
  while let Some(current_char) = iter.next() {
    if !in_dynamic && current_char == '<' {
      i += 1;
      if let Some('(') = iter.next() {
        i += 1;
        if let Some('{') = iter.next() {
          start = i+1;
          in_dynamic = true;
        }
      }
    } else if in_dynamic && current_char == '}' {
      i += 1;
      if let Some(')') = iter.next() {
        i += 1;
        if let Some('>') = iter.next() {
          result.push((start, i-2));
          in_dynamic = false;
        }
      }
    }
    i += 1;
  }
  result
}
