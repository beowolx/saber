use crate::{ast::Node, object::ObjectType};

#[cfg(test)]
mod tests {
  use crate::{lexer::Lexer, object::ObjectType, parser::Parser};

  fn test_eval(input: &str) -> ObjectType {
    let l = Lexer::new(input.to_owned());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();
    program.eval()
  }

  fn test_integer_object(obj: ObjectType, expected: i64) -> bool {
    match obj {
      ObjectType::Integer(value) => {
        if value != expected {
          panic!("object has wrong value. got={}, want={}", value, expected);
        }
        true
      }
      _ => {
        panic!("object is not Integer. got={:?}", obj);
      }
    }
  }

  #[test]
  fn test_eval_integer_expression() {
    let tests = vec![
      ("5", 5),
      ("10", 10),
      ("-5", -5),
      ("-10", -10),
      ("5 + 5 + 5 + 5 - 10", 10),
      ("2 * 2 * 2 * 2 * 2", 32),
      ("-50 + 100 + -50", 0),
      ("5 * 2 + 10", 20),
      ("5 + 2 * 10", 25),
      ("20 + 2 * -10", 0),
      ("50 / 2 * 2 + 10", 60),
      ("2 * (5 + 10)", 30),
      ("3 * 3 * 3 + 10", 37),
      ("3 * (3 * 3) + 10", 37),
      // ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
      let evaluated = test_eval(input);
      test_integer_object(evaluated, expected);
    }
  }
}
