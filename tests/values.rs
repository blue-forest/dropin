use dropin::parser::read_values;

#[test]
fn hello_world() {
  let ref_ = read_values(vec![
    "values blueforest:tests:v1:hello_world",
    "======================================",
    "format text",
    "data \"hello world\"",
  ].join("\n"));
  let mut buf = Vec::new();
  ref_.compile(&mut buf);
  let mut ground_truth = vec![
    1, // text instance
    0, // refs length
  ];
  ground_truth.push(11); // hello world length
  ground_truth.extend("hello world".as_bytes());
  assert_eq!(buf, ground_truth);
}
