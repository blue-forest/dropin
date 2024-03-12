use dropin_compiler_parser::parser::parse;
use dropin_compiler_parser::Table;

#[test]
fn example() {
	let table = Table::default();

	let input = "a == 1";
	parse(input, None, &table);
}
