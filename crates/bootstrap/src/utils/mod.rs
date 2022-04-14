pub fn escape_char(c: char) -> char {
  match c {
    'n' => '\n',
    't' => '\t',
    'r' => '\r',
    '0' => '\0',
    _ => c,
  }
}
