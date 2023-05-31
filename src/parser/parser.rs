use crate::{
  ast::{Identifier, Program, Statement, VarStatement},
  lexer::Lexer,
  token::Token,
  token::TokenType,
};

struct Parser {
  lexer: Lexer,
  current_token: Token,
  peek_token: Token,
}

impl Parser {
  pub fn new(mut l: Lexer) -> Self {
    let current_token = l.next_token();
    let peek_token = l.next_token();
    Self {
      lexer: l,
      current_token,
      peek_token,
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

  pub fn current_token_is(&self, token_type: TokenType) -> bool {
    self.current_token.token_type == token_type
  }

  pub fn peek_token_is(&self, token_type: TokenType) -> bool {
    self.peek_token.token_type == token_type
  }

  pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
    if self.peek_token_is(token_type) {
      self.next_token();
      true
    } else {
      false
    }
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

    let tests = vec!["x", "y", "foobar"];

    for (i, tt) in tests.iter().enumerate() {
      let stmt = &program.statements[i];
      assert_eq!(stmt.token_literal(), "var");
      // assert_eq!(
      //   stmt,
      //   &Statement::Var(VarStatement {
      //     token: Token::new(TokenType::Var, "let".to_string()),
      //     name: Identifier {
      //       token: Token::new(TokenType::Ident, tt.to_string()),
      //       value: tt.to_string(),
      //     },
      //     value: Expression::IntegerLiteral(IntegerLiteral {
      //       token: Token::new(TokenType::Int, "5".to_string()),
      //       value: 5,
      //     }),
      //   })
      // );
    }
  }
}
