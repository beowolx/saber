use std::{collections::HashMap, rc::Rc};

use crate::{
  ast::{
    Expression, Identifier, Program, RetStatement, Statement, VarStatement,
  },
  lexer::Lexer,
  token::Token,
  token::TokenType,
};

type PrefixParseFn = Rc<dyn Fn() -> dyn Expression>;
type InfixParseFn = Rc<dyn Fn(dyn Expression) -> dyn Expression>;

struct Parser {
  lexer: Lexer,
  current_token: Token,
  peek_token: Token,
  errors: Vec<String>,
  prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
  infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
  pub fn new(mut l: Lexer) -> Self {
    let current_token = l.next_token();
    let peek_token = l.next_token();
    let prefix_parse_fns = HashMap::new();
    let infix_parse_fns = HashMap::new();

    Self {
      lexer: l,
      current_token,
      peek_token,
      errors: vec![],
      prefix_parse_fns,
      infix_parse_fns,
    }
  }

  pub fn next_token(&mut self) {
    self.current_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
  }

  pub fn parse_program(&mut self) -> Option<Program> {
    let mut program = Program::new();
    while !self.current_token_is(TokenType::Eof) {
      if let Some(stmt) = self.parse_statement() {
        program.statements.push(stmt);
      }
      self.next_token();
    }
    Some(program)
  }

  pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
    match self.current_token.token_type {
      TokenType::Var => self.parse_var_statement(),
      TokenType::Ret => self.parse_ret_statement(),
      _ => None,
    }
  }

  pub fn parse_var_statement(&mut self) -> Option<Box<dyn Statement>> {
    let token = self.current_token.clone();

    if !self.expect_peek(TokenType::Ident) {
      return None;
    }

    let name = Identifier {
      token: self.current_token.clone(),
      value: self.current_token.literal.clone(),
    };

    if !self.expect_peek(TokenType::Assign) {
      return None;
    }

    // TODO: Skip the expression until we find a semicolon
    while !self.current_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    Some(Box::new(VarStatement {
      token,
      name,
      value: None,
    }))
  }

  pub fn parse_ret_statement(&mut self) -> Option<Box<dyn Statement>> {
    let token = self.current_token.clone();

    self.next_token();

    // TODO: Skip the expression until we find a semicolon
    while !self.current_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    Some(Box::new(RetStatement {
      token,
      return_value: None,
    }))
  }

  pub fn current_token_is(&self, token_type: TokenType) -> bool {
    self.current_token.token_type == token_type
  }

  pub fn peek_token_is(&self, token_type: TokenType) -> bool {
    self.peek_token.token_type == token_type
  }

  pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
    if self.peek_token_is(token_type.clone()) {
      self.next_token();
      true
    } else {
      self.peek_error(token_type);
      false
    }
  }

  pub fn errors(&self) -> Vec<String> {
    self.errors.clone()
  }

  pub fn peek_error(&mut self, token_type: TokenType) {
    let msg = format!(
      "expected next token to be {:?}, got {:?} instead",
      token_type, self.peek_token.token_type
    );
    self.errors.push(msg);
  }

  pub fn register_prefix(
    &mut self,
    token_type: TokenType,
    func: PrefixParseFn,
  ) {
    self.prefix_parse_fns.insert(token_type, func);
  }

  pub fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
    self.infix_parse_fns.insert(token_type, func);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lexer::Lexer;

  #[test]
  fn test_let_statements() {
    let input = "
        var x = 5;
        var y = 10;
        var foobar = 838383;
        ";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 3);

    for stmt in program.statements {
      assert_eq!(stmt.token_literal(), "var");
    }
  }

  #[test]
  fn test_ret_statements() {
    let input = "
        ret 5;
        ret 10;
        ret 993322;
        ";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 3);

    for stmt in program.statements {
      assert_eq!(stmt.token_literal(), "ret");
    }
  }

  #[test]
  fn test_parser_errors() {
    let input = "
        var x 5;
        var = 10;
        var 838383;
        ";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    p.parse_program().unwrap();

    assert_eq!(p.errors.len(), 3);

    dbg!(&p.errors);

    let errors = vec![
      "expected next token to be Assign, got Int instead",
      "expected next token to be Ident, got Assign instead",
      "expected next token to be Ident, got Int instead",
    ];

    for (i, err) in errors.iter().enumerate() {
      assert_eq!(p.errors[i], err.to_string());
    }
  }
}
