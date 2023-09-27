use crate::token::Token;

// Define a unified Node enum
#[derive(Debug, Clone)]
pub enum Node {
  Program(Program),
  Statement(Statement),
  Expression(Expression),
}

// Define Statement enum
#[derive(Debug, Clone)]
pub enum Statement {
  ForgeStatement(ForgeStatement),
  IgniteStatement(IgniteStatement),
  ExpressionStatement(ExpressionStatement),
  BlockStatement(BlockStatement),
}

// Define Expression enum
#[derive(Debug, Clone)]
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

// The existing structs remain mostly the same
pub struct Program {
  pub statements: Vec<Statement>,
}

pub struct ForgeStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<Expression>,
}

pub struct Identifier {
  pub token: Token,
  pub value: String,
}

pub struct IgniteStatement {
  pub token: Token,
  pub return_value: Option<Box<Expression>>,
}

pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<Box<Expression>>,
}

pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}

pub struct PrefixExpression {
  pub token: Token,
  pub operator: String,
  pub right: Option<Box<Expression>>,
}

pub struct InfixExpression {
  pub token: Token,
  pub left: Option<Box<Expression>>,
  pub operator: String,
  pub right: Option<Box<Expression>>,
}

pub struct Boolean {
  pub token: Token,
  pub value: bool,
}

pub struct BlockStatement {
  pub token: Token,
  pub statements: Vec<Box<Statement>>,
}

pub struct IfExpression {
  pub token: Token,
  pub condition: Option<Box<Expression>>,
  pub consequence: Option<BlockStatement>,
  pub alternative: Option<BlockStatement>,
}

pub struct FunctionLiteral {
  pub token: Token,
  pub parameters: Vec<Identifier>,
  pub body: Option<BlockStatement>,
}

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
