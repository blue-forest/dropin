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

use dropin_compiler_common::ir::{
  Comparison, Control, Expression, Logic, Value,
};
use dropin_compiler_parser_lib::{parse, Table};
use indoc::indoc;

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

#[test]
fn logic_priority_and() {
  let table = Table::default();
  let input = "true & false | false";
  let expr = parse(&mut Printer, input.into(), None, &table);
  let Expression::Logic(Logic::Or(or)) = expr else {
    assert!(false, r#""{input}" ==> {expr:?}"#);
    return;
  };
  assert!(or.len() == 2, "wrong or: {or:?}");
  let Expression::Logic(Logic::And(and)) = &or[0] else {
    assert!(false, "wrong or first operand: {:?}", or[0]);
    return;
  };
  assert!(and.len() == 2, "wrong and: {and:?}");
  let Expression::Value(Value::Boolean(true)) = and[0] else {
    assert!(false, "wrong and first operand: {:?}", and[0]);
    return;
  };
  let Expression::Value(Value::Boolean(false)) = and[1] else {
    assert!(false, "wrong and second operand: {:?}", and[1]);
    return;
  };
  let Expression::Value(Value::Boolean(false)) = or[1] else {
    assert!(false, "wrong or second operand: {:?}", or[1]);
    return;
  };
}

#[test]
fn logic_priority_or() {
  let table = Table::default();
  let input = "false | true & false";
  let expr = parse(&mut Printer, input.into(), None, &table);
  let Expression::Logic(Logic::And(and)) = expr else {
    assert!(false, r#""{input}" ==> {expr:?}"#);
    return;
  };
  assert!(and.len() == 2, "wrong or: {and:?}");
  let Expression::Logic(Logic::Or(or)) = &and[0] else {
    assert!(false, "wrong and first operand: {:?}", and[0]);
    return;
  };
  assert!(or.len() == 2, "wrong and: {or:?}");
  let Expression::Value(Value::Boolean(false)) = or[0] else {
    assert!(false, "wrong or first operand: {:?}", or[0]);
    return;
  };
  let Expression::Value(Value::Boolean(true)) = or[1] else {
    assert!(false, "wrong or second operand: {:?}", or[1]);
    return;
  };
  let Expression::Value(Value::Boolean(false)) = and[1] else {
    assert!(false, "wrong and second operand: {:?}", and[1]);
    return;
  };
}

#[test]
fn simple_if() {
  let table = Table::default();
  let input = r#"if a == "test": false else: true"#;
  let expr = parse(&mut Printer, input.into(), None, &table);
  let Expression::Control(Control::If(if_)) = expr else {
    assert!(false, r#""{input}" ==> {expr:?}"#);
    return;
  };
  let Expression::Comparison(Comparison::EqualsTo(
    condition_left,
    condition_right,
  )) = if_.condition.as_ref()
  else {
    assert!(false, "wrong condition: {:?}", if_.condition);
    return;
  };
  let Expression::Value(Value::Getter(ident, indexes)) =
    condition_left.as_ref()
  else {
    assert!(false, "wrong condition left: {:?}", condition_left);
    return;
  };
  assert!(ident == "a", r#"wrong identifier: "{ident}""#);
  assert!(indexes.is_empty(), "unexpected indexes: {indexes:?}");
  let Expression::Value(Value::Text(condition_right_text)) =
    condition_right.as_ref()
  else {
    assert!(false, "wrong condition right: {:?}", condition_right);
    return;
  };
  assert!(
    condition_right_text == "test",
    r#"wrong condition right text: "{condition_right_text}""#
  );
  let Expression::Value(Value::Boolean(false)) = if_.then.as_ref() else {
    assert!(false, "wrong then: {:?}", if_.then);
    return;
  };
  let Some(&Expression::Value(Value::Boolean(true))) =
    if_.else_.as_ref().map(|e| e.as_ref())
  else {
    assert!(false, "wrong else: {:?}", if_.else_);
    return;
  };
}

#[test]
fn reverse() {
  let table = Table::default();
  let input = "reverse{\
      list, current, result:\
        if current < len(list):\
          reverse(\
            list,\
            (current + 1),\
            push(result, list[(len(list) - current)])\
          )\
        else: result\
      }";
  let expr = parse(&mut Printer, input.into(), None, &table);
  println!("{expr:?}");
}
