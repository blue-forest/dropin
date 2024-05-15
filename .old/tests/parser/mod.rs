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
  Arithmetic, Comparison, Control, Expression, Logic, Value,
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
  let input = indoc!(
    "reverse{
      list, current, result:\
      if current < len(list):
        reverse(\
          list,\
          (current + 1),\
          push(result, list[(len(list) - current)])\
        )
      else: result\
    }(
        1
        2
        3
        4
      ,\
      0,\
      []\
    )"
  );
  let expr = parse(&mut Printer, input.into(), None, &table);
  let Expression::Control(Control::FunctionCall(function_call)) = expr else {
    assert!(false, r#""{input}" ==> {expr:?}"#);
    return;
  };
  let Expression::Control(Control::NamedFunction(function)) =
    function_call.function.as_ref()
  else {
    assert!(false, "wrong function called: {:?}", function_call.function);
    return;
  };
  assert!(
    function.name == "reverse",
    "wrong function name: {}",
    function.name
  );
  assert!(function.args.len() == 3, "wrong args: {:?}", function.args);
  assert!(
    function.args[0] == "list",
    "wrong first arg: {}",
    function.args[0]
  );
  assert!(
    function.args[1] == "current",
    "wrong second arg: {}",
    function.args[1]
  );
  assert!(
    function.args[2] == "result",
    "wrong third arg: {}",
    function.args[2]
  );
  // body
  {
    let Expression::Control(Control::If(if_)) = function.body.as_ref() else {
      assert!(false, "wrong function body: {:?}", function.body);
      return;
    };
    // condition
    {
      let Expression::Comparison(Comparison::LessThan(left, right)) =
        if_.condition.as_ref()
      else {
        assert!(false, "wrong if condition: {:?}", if_.condition);
        return;
      };
      let Expression::Value(Value::Getter(ident, indexes)) = left.as_ref()
      else {
        assert!(false, "wrong less than left side: {:?}", left);
        return;
      };
      assert!(ident == "current", "wrong condition current ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong condition current indexes: {:?}",
        indexes
      );
      let Expression::Control(Control::FunctionCall(function_call)) =
        right.as_ref()
      else {
        assert!(false, "wrong less than right side: {:?}", right);
        return;
      };
      let Expression::Value(Value::Getter(ident, indexes)) =
        function_call.function.as_ref()
      else {
        assert!(
          false,
          "wrong less condition len call: {:?}",
          function_call.function
        );
        return;
      };
      assert!(ident == "len", "wrong condition len ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong condition len indexes: {indexes:?}"
      );
      assert!(
        function_call.args.len() == 1,
        "wrong condition len args: {:?}",
        function_call.args
      );
      let Expression::Value(Value::Getter(ident, indexes)) =
        &function_call.args[0]
      else {
        assert!(
          false,
          "wrong condition len arg: {:?}",
          function_call.args[0]
        );
        return;
      };
      assert!(ident == "list", "wrong condition len arg ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong condition len arg indexes: {indexes:?}"
      );
    } // condition
      // then
    {
      let Expression::Control(Control::FunctionCall(function_call)) =
        if_.then.as_ref()
      else {
        assert!(false, "wrong if then: {:?}", if_.then);
        return;
      };
      let Expression::Value(Value::Getter(ident, indexes)) =
        function_call.function.as_ref()
      else {
        assert!(
          false,
          "wrong recursion call function: {:?}",
          function_call.function
        );
        return;
      };
      assert!(ident == "reverse", "wrong recursion call ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong recursion call indexes: {indexes:?}"
      );
      assert!(
        function_call.args.len() == 3,
        "wrong recursion call args: {:?}",
        function_call.args
      );
      let Expression::Value(Value::Getter(ident, indexes)) =
        &function_call.args[0]
      else {
        assert!(
          false,
          "wrong recursion call first arg: {:?}",
          function_call.args[0]
        );
        return;
      };
      assert!(
        ident == "list",
        "wrong recursion call first arg ident: {ident}"
      );
      assert!(
        indexes.is_empty(),
        "wrong recursion call first arg indexes: {indexes:?}"
      );
      let Expression::Arithmetic(Arithmetic::Add(left, right)) =
        &function_call.args[1]
      else {
        assert!(
          false,
          "wrong recursion call second arg: {:?}",
          function_call.args[1]
        );
        return;
      };
      let Expression::Value(Value::Getter(ident, indexes)) = left.as_ref()
      else {
        assert!(false, "wrong recursion call second arg left: {left:?}");
        return;
      };
      assert!(
        ident == "current",
        "wrong recursion call second arg left ident: {ident}"
      );
      assert!(
        indexes.is_empty(),
        "wrong recursion call second arg left indexes: {indexes:?}"
      );
      let Expression::Value(Value::Quantity(value)) = right.as_ref() else {
        assert!(false, "wrong recursion call second arg right: {right:?}");
        return;
      };
      assert!(
        *value == 1.0,
        "wrong recursion call second arg right value: {value}"
      );
      let Expression::Control(Control::FunctionCall(function_call)) =
        &function_call.args[2]
      else {
        assert!(
          false,
          "wrong recursion call third arg: {:?}",
          function_call.args[2]
        );
        return;
      };
      let Expression::Value(Value::Getter(ident, indexes)) =
        function_call.function.as_ref()
      else {
        assert!(false, "wrong push function: {:?}", function_call.function);
        return;
      };
      assert!(ident == "push", "wrong push function ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong push function indexes: {indexes:?}"
      );
      assert!(
        function_call.args.len() == 2,
        "wrong push args: {:?}",
        function_call.args
      );
      let Expression::Value(Value::Getter(ident, indexes)) =
        &function_call.args[0]
      else {
        assert!(false, "wrong push first arg: {:?}", function_call.args[0]);
        return;
      };
      assert!(ident == "result", "wrong push first arg ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong push first arg indexes: {indexes:?}"
      );
      let Expression::Value(Value::Getter(ident, indexes)) =
        &function_call.args[1]
      else {
        assert!(false, "wrong push second arg: {:?}", function_call.args[1]);
        return;
      };
      assert!(ident == "list", "wrong push second arg ident: {ident}");
      assert!(
        indexes.len() == 1,
        "wrong push second arg indexes: {indexes:?}"
      );
      let Expression::Arithmetic(Arithmetic::Sub(left, right)) = &indexes[0]
      else {
        assert!(false, "wrong list index: {:?}", indexes[0]);
        return;
      };
      let Expression::Control(Control::FunctionCall(function_call)) =
        left.as_ref()
      else {
        assert!(false, "wrong list index left: {left:?}");
        return;
      };
      let Expression::Value(Value::Getter(ident, indexes)) =
        function_call.function.as_ref()
      else {
        assert!(
          false,
          "wrong list index left function: {:?}",
          function_call.function
        );
        return;
      };
      assert!(
        ident == "len",
        "wrong list index left function ident: {ident}"
      );
      assert!(
        indexes.is_empty(),
        "wrong list index left function indexes: {indexes:?}"
      );
      assert!(
        function_call.args.len() == 1,
        "wrong list index left args: {:?}",
        function_call.args
      );
      let Expression::Value(Value::Getter(ident, indexes)) =
        &function_call.args[0]
      else {
        assert!(
          false,
          "wrong list index left arg: {:?}",
          function_call.args[0]
        );
        return;
      };
      assert!(ident == "list", "wrong list index left arg ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong list index left arg indexes: {indexes:?}"
      );
      let Expression::Value(Value::Getter(ident, indexes)) = right.as_ref()
      else {
        assert!(false, "wrong list index right: {right:?}");
        return;
      };
      assert!(ident == "current", "wrong list index right ident: {ident}");
      assert!(
        indexes.is_empty(),
        "wrong list index right indexes: {indexes:?}"
      );
    } // then
      // else
    {
      let Some(Expression::Value(Value::Getter(ident, indexes))) =
        if_.else_.as_ref().map(|expr| expr.as_ref())
      else {
        assert!(false, "wrong else: {:?}", if_.else_);
        return;
      };
      assert!(ident == "result", "wrong else ident: {ident}");
      assert!(indexes.is_empty(), "wrong else indexes: {indexes:?}");
    } // else
  } // body
    // args
  {
    assert!(
      function_call.args.len() == 3,
      "wrong args: {:?}",
      function_call.args
    );
    let Expression::Value(Value::List(list)) = &function_call.args[0] else {
      assert!(false, "wrong first arg: {:?}", function_call.args[0]);
      return;
    };
    assert!(list.len() == 4, "wrong list: {list:?}");
    let Expression::Value(Value::Quantity(value)) = &list[0] else {
      assert!(false, "wrong list[0]: {:?}", list[0]);
      return;
    };
    assert!(*value == 1.0, "wrong list[0] value: {value}");
    let Expression::Value(Value::Quantity(value)) = &list[1] else {
      assert!(false, "wrong list[1]: {:?}", list[1]);
      return;
    };
    assert!(*value == 2.0, "wrong list[1] value: {value}");
    let Expression::Value(Value::Quantity(value)) = &list[2] else {
      assert!(false, "wrong list[2]: {:?}", list[2]);
      return;
    };
    assert!(*value == 3.0, "wrong list[2] value: {value}");
    let Expression::Value(Value::Quantity(value)) = &list[3] else {
      assert!(false, "wrong list[3]: {:?}", list[3]);
      return;
    };
    assert!(*value == 4.0, "wrong list[3] value: {value}");
    let Expression::Value(Value::Quantity(value)) = &function_call.args[1]
    else {
      assert!(false, "wrong second arg: {:?}", function_call.args[1]);
      return;
    };
    assert!(*value == 0.0, "wrong current value: {value}");
    let Expression::Value(Value::List(result)) = &function_call.args[2] else {
      assert!(false, "wrong third arg: {:?}", function_call.args[2]);
      return;
    };
    assert!(result.is_empty(), "wrong result: {result:?}");
  } // args
}
