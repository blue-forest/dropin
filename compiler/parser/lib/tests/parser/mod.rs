/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler - WebAssembly
 *              |_|
 * Copyright © 2019-2024 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use dropin_compiler_common::ir::{Comparison, Expression, Value};
use dropin_compiler_parser_lib::{parse, Table};

use crate::common::Printer;

#[test]
fn example() {
  let table = Table::default();

  let input = "a == 1";
  let expr = parse(&mut Printer, input.into(), None, &table);
  let Expression::Comparison(Comparison::EqualsTo(left, right)) = &expr else {
    assert!(false, r#""{input}" ==> {expr:?}"#);
    return;
  };
  let Expression::Value(Value::Getter(ident, indexes)) = left.as_ref() else {
    assert!(false, r#"wrong left value: "{input}" ==> {expr:?}"#);
    return;
  };
  assert!(ident == "a", "wrong identifier: {ident}");
  assert!(indexes.is_empty(), "indexes are not empty: {indexes:?}");
  let Expression::Value(Value::Quantity(quantity)) = right.as_ref() else {
    assert!(false, r#"wrong right value: "{input}" ==> {expr:?}"#);
    return;
  };
  assert!(*quantity == 1.0, "wrong quantity: {quantity}");
}
