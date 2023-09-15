use crate::token::Token;

pub trait Node {
  fn token_literal(&self) -> String;
  fn string(&self) -> String;
}

pub trait Statement: Node {
  fn statement_node(&self);
}

pub trait Expression: Node {
  fn expression_node(&self);
}

pub struct Program {
  pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
  pub fn new() -> Self {
    Self {
      statements: Vec::new(),
    }
  }
}

impl Node for Program {
  fn token_literal(&self) -> String {
    if !self.statements.is_empty() {
      self.statements[0].token_literal()
    } else {
      String::from("")
    }
  }

  fn string(&self) -> String {
    let mut out = String::new();

    for s in &self.statements {
      out.push_str(&s.string());
    }

    out
  }
}

pub struct ForgeStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<Box<dyn Expression>>,
}

impl Statement for ForgeStatement {
  fn statement_node(&self) {}
}

impl Node for ForgeStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    let mut out = String::new();

    out.push_str(&self.token_literal());
    out.push(' ');
    out.push_str(&self.name.value);
    out.push_str(" = ");

    if let Some(value) = &self.value {
      out.push_str(&value.string());
    }

    out.push(';');

    out
  }
}

pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Expression for Identifier {
  fn expression_node(&self) {}
}

impl Node for Identifier {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    self.value.clone()
  }
}

pub struct IgniteStatement {
  pub token: Token,
  pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for IgniteStatement {
  fn statement_node(&self) {}
}

impl Node for IgniteStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    out.push_str(&self.token_literal());
    out.push(' ');

    if let Some(value) = &self.return_value {
      out.push_str(&value.string());
    }

    out.push(';');

    out
  }
}

pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
  fn statement_node(&self) {}
}

impl Node for ExpressionStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    if let Some(expr) = &self.expression {
      expr.string()
    } else {
      String::new()
    }
  }
}

pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}

impl Expression for IntegerLiteral {
  fn expression_node(&self) {}
}

impl Node for IntegerLiteral {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    self.token.literal.clone()
  }
}

pub struct PrefixExpression {
  pub token: Token,
  pub operator: String,
  pub right: Option<Box<dyn Expression>>,
}

impl Expression for PrefixExpression {
  fn expression_node(&self) {}
}

impl Node for PrefixExpression {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    let mut out = String::new();

    out.push('(');
    out.push_str(&self.operator);

    if let Some(right) = &self.right {
      out.push_str(&right.string());
    }

    out.push(')');

    out
  }
}
pub struct InfixExpression {
  pub token: Token,
  pub left: Option<Box<dyn Expression>>,
  pub operator: String,
  pub right: Option<Box<dyn Expression>>,
}

impl Expression for InfixExpression {
  fn expression_node(&self) {}
}

impl Node for InfixExpression {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    out.push('(');

    if let Some(left) = &self.left {
      out.push_str(&left.string());
    }

    out.push(' ');
    out.push_str(&self.operator);
    out.push(' ');

    if let Some(right) = &self.right {
      out.push_str(&right.string());
    }

    out.push(')');

    out
  }
}

pub struct Boolean {
  pub token: Token,
  pub value: bool,
}

impl Expression for Boolean {
  fn expression_node(&self) {}
}

impl Node for Boolean {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
  fn string(&self) -> String {
    self.token.literal.clone()
  }
}

pub struct BlockStatement {
  pub token: Token,
  pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
  fn statement_node(&self) {}
}

impl Node for BlockStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    for s in &self.statements {
      out.push_str(&s.string());
    }

    out
  }
}

pub struct IfExpression {
  pub token: Token,
  pub condition: Option<Box<dyn Expression>>,
  pub consequence: Option<BlockStatement>,
  pub alternative: Option<BlockStatement>,
}

impl Expression for IfExpression {
  fn expression_node(&self) {}
}

impl Node for IfExpression {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    out.push_str("if");
    out.push(' ');

    if let Some(condition) = &self.condition {
      out.push_str(&condition.string());
    }

    out.push(' ');

    if let Some(consequence) = &self.consequence {
      out.push_str(&consequence.string());
    }

    if let Some(alternative) = &self.alternative {
      out.push_str("else ");
      out.push_str(&alternative.string());
    }

    out
  }
}

pub struct FunctionLiteral {
  pub token: Token,
  pub parameters: Vec<Identifier>,
  pub body: Option<BlockStatement>,
}

impl Expression for FunctionLiteral {
  fn expression_node(&self) {}
}

impl Node for FunctionLiteral {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    let params: Vec<String> =
      self.parameters.iter().map(|p| p.string()).collect();

    out.push_str(&self.token_literal());
    out.push('(');
    out.push_str(&params.join(", "));
    out.push(')');

    if let Some(body) = &self.body {
      out.push_str(&body.string());
    }

    out
  }
}

pub struct CallExpression {
  pub token: Token,
  pub function: Option<Box<dyn Expression>>,
  pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {
  fn expression_node(&self) {}
}

impl Node for CallExpression {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut out = String::new();

    if let Some(function) = &self.function {
      out.push_str(&function.string());
    }

    out.push('(');

    let args: Vec<String> = self.arguments.iter().map(|a| a.string()).collect();

    out.push_str(&args.join(", "));

    out.push(')');

    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::TokenType;

  #[test]
  fn test_string_value() {
    let program = Program {
      statements: vec![Box::new(ForgeStatement {
        token: Token {
          token_type: TokenType::Forge,
          literal: String::from("forge"),
        },
        name: Identifier {
          token: Token {
            token_type: TokenType::Ident,
            literal: String::from("myForge"),
          },
          value: String::from("myForge"),
        },
        value: Some(Box::new(Identifier {
          token: Token {
            token_type: TokenType::Ident,
            literal: String::from("anotherForge"),
          },
          value: String::from("anotherForge"),
        })),
      })],
    };

    assert_eq!(program.string(), "forge myForge = anotherForge;");
  }
}
