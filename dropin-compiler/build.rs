use std::fs::{create_dir, read_to_string, write};
use std::ops::Index;

fn main() {
  let mut main = read_to_string("parser/types.pest").unwrap();
  replace_dynamics(&mut main);
  let _ = create_dir("../target/parser");
  println!("{:?}", std::env::current_dir().unwrap());
  write("../target/parser/types.pest", main).unwrap();
}

fn replace_dynamics(content: &mut String) {
  let dynamics = get_dynamics_and_remove_comments(content);
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

fn get_dynamics_and_remove_comments(content: &mut String) -> Vec<(usize, usize)> {
  let mut remove_at = Vec::new();
  let mut result = Vec::new();
  {
    let mut i = 0;
    let mut start = 0;
    let mut in_dynamic = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut iter = content.bytes();
    while let Some(current_char) = iter.next() {
      if !in_dynamic && !in_line_comment && !in_block_comment {
        match current_char {
          b'<' => {
            i += 1;
            if let Some(b'(') = iter.next() {
              i += 1;
              if let Some(b'{') = iter.next() {
                start = i+1;
                in_dynamic = true;
              }
            }
          }
          b'/' => {
            i += 1;
            if let Some(current_char) = iter.next() {
              match current_char {
                b'/' => {
                  start = i-1;
                  in_line_comment = true;
                }
                b'*' => {
                  start = i-1;
                  in_block_comment = true;
                }
                _ => {}
              }
            }
          }
          _ => {}
        }
      } else if in_dynamic && current_char == b'}' {
        i += 1;
        if let Some(b')') = iter.next() {
          i += 1;
          if let Some(b'>') = iter.next() {
            result.push((start, i-2));
            in_dynamic = false;
          }
        }
      } else if in_line_comment && current_char == b'\n' {
        in_line_comment = false;
        remove_at.push((start, i));
        i = start;
      } else if in_block_comment && current_char == b'*' {
        i += 1;
        if let Some(b'/') = iter.next() {
          in_block_comment = false;
          remove_at.push((start, i+1));
          i = start;
          continue
        }
      }
      i += 1;
    }
  }
  for (start, end) in remove_at.iter() {
    content.replace_range(start..end, "");
  }
  result
}
