/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
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
fn ident_equals_to_quantity() {
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

#[test]
fn nested_equals_to_quantity() {
  let table = Table::default();

  let input = "a.b[3].c[4] == 5";
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
  assert!(indexes.len() == 4, "wrong indexes: {indexes:?}");
  let Expression::Value(Value::Text(key1)) = &indexes[0] else {
    assert!(false, "wrong key 1: {:?}", indexes[0]);
    return;
  };
  assert!(key1 == "b", "wrong key 1: {key1}");
  let Expression::Value(Value::Quantity(key2)) = &indexes[1] else {
    assert!(false, "wrong key 2: {:?}", indexes[1]);
    return;
  };
  assert!(*key2 == 3.0, "wrong key 2: {key2}");
  let Expression::Value(Value::Text(key3)) = &indexes[2] else {
    assert!(false, "wrong key 3: {:?}", indexes[2]);
    return;
  };
  assert!(key3 == "c", "wrong key 3: {key3}");
  let Expression::Value(Value::Quantity(key4)) = &indexes[3] else {
    assert!(false, "wrong key 4: {:?}", indexes[3]);
    return;
  };
  assert!(*key4 == 4.0, "wrong key 4: {key4}");
  let Expression::Value(Value::Quantity(quantity)) = right.as_ref() else {
    assert!(false, r#"wrong right value: "{input}" ==> {expr:?}"#);
    return;
  };
  assert!(*quantity == 5.0, "wrong quantity: {quantity}");
}
