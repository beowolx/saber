use crate::token::Token;

pub enum Node {
  Program(Program),
  Statement(Statement),
  Expression(Expression),
}

// Define Statement enum
#[derive(Debug)]
pub enum Statement {
  ForgeStatement(ForgeStatement),
  IgniteStatement(IgniteStatement),
  ExpressionStatement(ExpressionStatement),
  BlockStatement(BlockStatement),
}

// Define Expression enum
#[derive(Debug)]
pub enum Expression {
  Identifier(Identifier),
  IntegerLiteral(IntegerLiteral),
  PrefixExpression(PrefixExpression),
  InfixExpression(InfixExpression),
  Boolean(Boolean),
  IfExpression(IfExpression),
  FunctionLiteral(FunctionLiteral),
  CallExpression(CallExpression),
}

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>,
}
use std::fmt;

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut program_string = String::new();
    for statement in &self.statements {
      program_string.push_str(&format!("{:?}\n", statement));
    }
    write!(f, "{}", program_string)
  }
}

impl Program {
  pub fn new() -> Program {
    Program {
      statements: Vec::new(),
    }
  }
}

#[derive(Debug)]
pub struct ForgeStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}
#[derive(Debug)]
pub struct IgniteStatement {
  pub token: Token,
  pub return_value: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}

#[derive(Debug)]
pub struct PrefixExpression {
  pub token: Token,
  pub operator: String,
  pub right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct InfixExpression {
  pub token: Token,
  pub left: Option<Box<Expression>>,
  pub operator: String,
  pub right: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct Boolean {
  pub token: Token,
  pub value: bool,
}

#[derive(Debug)]
pub struct BlockStatement {
  pub token: Token,
  pub statements: Vec<Box<Statement>>,
}

#[derive(Debug)]
pub struct IfExpression {
  pub token: Token,
  pub condition: Option<Box<Expression>>,
  pub consequence: Option<BlockStatement>,
  pub alternative: Option<BlockStatement>,
}

#[derive(Debug)]
pub struct FunctionLiteral {
  pub token: Token,
  pub parameters: Vec<Identifier>,
  pub body: Option<BlockStatement>,
}

#[derive(Debug)]
pub struct CallExpression {
  pub token: Token,
  pub function: Option<Box<Expression>>,
  pub arguments: Vec<Box<Expression>>,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::TokenType;

  #[test]
  fn test_string_value() {
    let program = Program {
      statements: vec![Statement::ForgeStatement(ForgeStatement {
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
        value: Some(Expression::Identifier(Identifier {
          token: Token {
            token_type: TokenType::Ident,
            literal: String::from("anotherForge"),
          },
          value: String::from("anotherForge"),
        })),
      })],
    };

    assert_eq!(program.to_string(), "forge myForge = anotherForge;");
  }
}
