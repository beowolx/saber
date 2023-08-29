use crate::{
  ast::{
    BlockStatement, Expression, ExpressionStatement, FunctionLiteral,
    Identifier, IfExpression, InfixExpression, IntegerLiteral,
    PrefixExpression, Program, RetStatement, Statement, VarStatement,
  },
  lexer::Lexer,
  token::Token,
  token::TokenType,
};
use std::{collections::HashMap, vec};

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn =
  fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

#[derive(Eq, PartialEq, PartialOrd)]
enum Precedence {
  Lowest,
  Equals,
  LessGreater,
  Sum,
  Product,
  Prefix,
  Call,
}

struct Parser {
  lexer: Lexer,
  current_token: Token,
  peek_token: Token,
  errors: Vec<String>,
  prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
  infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

const PRECEDENCES: [(TokenType, Precedence); 8] = [
  (TokenType::Eq, Precedence::Equals),
  (TokenType::NotEq, Precedence::Equals),
  (TokenType::Lt, Precedence::LessGreater),
  (TokenType::Gt, Precedence::LessGreater),
  (TokenType::Plus, Precedence::Sum),
  (TokenType::Minus, Precedence::Sum),
  (TokenType::Slash, Precedence::Product),
  (TokenType::Asterisk, Precedence::Product),
];

impl Parser {
  fn new(mut l: Lexer) -> Self {
    let current_token = l.next_token();
    let peek_token = l.next_token();
    let mut prefix_parse_fns: HashMap<TokenType, PrefixParseFn> =
      HashMap::new();
    prefix_parse_fns.insert(TokenType::Ident, Self::parse_identifier);
    prefix_parse_fns.insert(TokenType::Int, Self::parse_integer_literal);
    prefix_parse_fns.insert(TokenType::Bang, Self::parse_prefix_expression);
    prefix_parse_fns.insert(TokenType::Minus, Self::parse_prefix_expression);
    prefix_parse_fns.insert(TokenType::True, Self::parse_boolean);
    prefix_parse_fns.insert(TokenType::False, Self::parse_boolean);
    prefix_parse_fns.insert(TokenType::Lparen, Self::parse_grouped_expression);
    prefix_parse_fns.insert(TokenType::If, Self::parse_if_expression);
    prefix_parse_fns.insert(TokenType::Function, Self::parse_function_literal);

    let mut infix_parse_fns: HashMap<TokenType, InfixParseFn> = HashMap::new();
    infix_parse_fns.insert(TokenType::Plus, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::Minus, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::Slash, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::Asterisk, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::Eq, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::NotEq, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::Lt, Self::parse_infix_expression);
    infix_parse_fns.insert(TokenType::Gt, Self::parse_infix_expression);

    Self {
      lexer: l,
      current_token,
      peek_token,
      errors: vec![],
      prefix_parse_fns,
      infix_parse_fns,
    }
  }

  fn next_token(&mut self) {
    self.current_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
  }

  fn parse_program(&mut self) -> Option<Program> {
    let mut program = Program::new();
    while !self.current_token_is(TokenType::Eof) {
      if let Some(stmt) = self.parse_statement() {
        program.statements.push(stmt);
      }
      self.next_token();
    }
    Some(program)
  }

  fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
    match self.current_token.token_type {
      TokenType::Var => self.parse_var_statement(),
      TokenType::Ret => self.parse_ret_statement(),
      _ => self.parse_expression_statement(),
    }
  }

  fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
    Some(Box::new(Identifier {
      token: self.current_token.clone(),
      value: self.current_token.literal.clone(),
    }))
  }

  fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
    match self.current_token.literal.parse::<i64>() {
      Ok(value) => Some(Box::new(IntegerLiteral {
        token: self.current_token.clone(),
        value,
      })),
      Err(_) => None,
    }
  }

  fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
    let token = self.current_token.clone();
    let operator = self.current_token.literal.clone();

    self.next_token();

    if let Some(right) = self.parse_expression(Precedence::Prefix) {
      Some(Box::new(PrefixExpression {
        token,
        operator,
        right: Some(right),
      }))
    } else {
      None
    }
  }

  fn parse_infix_expression(
    &mut self,
    left: Box<dyn Expression>,
  ) -> Option<Box<dyn Expression>> {
    let token = self.current_token.clone();
    let operator = self.current_token.literal.clone();
    let precedence = self.current_precedence();
    self.next_token();

    let right = self.parse_expression(precedence).unwrap();
    Some(Box::new(InfixExpression {
      token,
      left: Some(left),
      operator,
      right: Some(right),
    }))
  }

  fn parse_var_statement(&mut self) -> Option<Box<dyn Statement>> {
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

  fn parse_ret_statement(&mut self) -> Option<Box<dyn Statement>> {
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

  fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
    let token = self.current_token.clone();
    let expression = self.parse_expression(Precedence::Lowest);

    if self.peek_token_is(TokenType::Semicolon) {
      self.next_token();
    }

    Some(Box::new(ExpressionStatement { token, expression }))
  }

  fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
    Some(Box::new(Identifier {
      token: self.current_token.clone(),
      value: self.current_token_is(TokenType::True).to_string(),
    }))
  }

  fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
    self.next_token();

    let exp = self.parse_expression(Precedence::Lowest);

    if !self.expect_peek(TokenType::Rparen) {
      return None;
    }

    exp
  }

  fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
    let token = self.current_token.clone();

    if !self.expect_peek(TokenType::Lparen) {
      return None;
    }

    self.next_token();

    let condition = self.parse_expression(Precedence::Lowest);

    if !self.expect_peek(TokenType::Rparen) {
      return None;
    }

    if !self.expect_peek(TokenType::Lbrace) {
      return None;
    }

    let consequence = self.parse_block_statement();

    let alternative = if self.peek_token_is(TokenType::Else) {
      self.next_token();

      if !self.expect_peek(TokenType::Lbrace) {
        return None;
      }
      self.parse_block_statement()
    } else {
      None
    };

    Some(Box::new(IfExpression {
      token,
      condition,
      consequence,
      alternative,
    }))
  }

  fn parse_function_literal(&mut self) -> Option<Box<dyn Expression>> {
    let token = self.current_token.clone();

    if !self.expect_peek(TokenType::Lparen) {
      return None;
    }

    let parameters = self.parse_function_parameters();

    if !self.expect_peek(TokenType::Lbrace) {
      return None;
    }

    let body = self.parse_block_statement();

    Some(Box::new(FunctionLiteral {
      token,
      parameters,
      body,
    }))
  }

  fn parse_function_parameters(&mut self) -> Vec<Identifier> {
    let mut identifiers = vec![];

    if self.peek_token_is(TokenType::Rparen) {
      self.next_token();
      return identifiers;
    }

    self.next_token();

    let ident = Identifier {
      token: self.current_token.clone(),
      value: self.current_token.literal.clone(),
    };

    identifiers.push(ident);

    while self.peek_token_is(TokenType::Comma) {
      self.next_token();
      self.next_token();

      let ident = Identifier {
        token: self.current_token.clone(),
        value: self.current_token.literal.clone(),
      };

      identifiers.push(ident);
    }

    if !self.expect_peek(TokenType::Rparen) {
      return vec![];
    }

    identifiers
  }

  fn parse_block_statement(&mut self) -> Option<BlockStatement> {
    let mut block = BlockStatement {
      token: self.current_token.clone(),
      statements: vec![],
    };

    self.next_token();

    while !self.current_token_is(TokenType::Rbrace)
      && !self.current_token_is(TokenType::Eof)
    {
      if let Some(stmt) = self.parse_statement() {
        block.statements.push(stmt);
      }
      self.next_token();
    }

    Some(block)
  }

  fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
    let msg = format!("no prefix parse function for {:?} found", token_type);
    self.errors.push(msg);
  }

  fn parse_expression(
    &mut self,
    precedence: Precedence,
  ) -> Option<Box<dyn Expression>> {
    let prefix = self.prefix_parse_fns.get(&self.current_token.token_type);

    if prefix.is_none() {
      self.no_prefix_parse_fn_error(self.current_token.token_type.clone());
      return None;
    }

    let mut left_exp = prefix.unwrap()(self).unwrap();

    while !self.peek_token_is(TokenType::Semicolon)
      && precedence < self.peek_precedence()
    {
      let token_type = self.peek_token.token_type.clone();

      let infix_fn_option = self.infix_parse_fns.get(&token_type).cloned();

      self.next_token();

      if let Some(infix_fn) = infix_fn_option {
        left_exp = infix_fn(self, left_exp).unwrap();
      } else {
        return Some(left_exp);
      }
    }

    Some(left_exp)
  }

  fn current_token_is(&self, token_type: TokenType) -> bool {
    self.current_token.token_type == token_type
  }

  fn peek_token_is(&self, token_type: TokenType) -> bool {
    self.peek_token.token_type == token_type
  }

  fn peek_precedence(&self) -> Precedence {
    for (token_type, precedence) in PRECEDENCES.into_iter() {
      if self.peek_token_is(token_type) {
        return precedence;
      }
    }
    Precedence::Lowest
  }

  fn current_precedence(&self) -> Precedence {
    for (token_type, precedence) in PRECEDENCES.into_iter() {
      if self.current_token_is(token_type) {
        return precedence;
      }
    }
    Precedence::Lowest
  }

  fn expect_peek(&mut self, token_type: TokenType) -> bool {
    if self.peek_token_is(token_type.clone()) {
      self.next_token();
      true
    } else {
      self.peek_error(token_type);
      false
    }
  }

  fn errors(&self) -> Vec<String> {
    self.errors.clone()
  }

  fn peek_error(&mut self, token_type: TokenType) {
    let msg = format!(
      "expected next token to be {:?}, got {:?} instead",
      token_type, self.peek_token.token_type
    );
    self.errors.push(msg);
  }

  fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
    self.prefix_parse_fns.insert(token_type, func);
  }

  fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
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

    assert_eq!(p.errors.len(), 4);

    let errors = [
      "expected next token to be Assign, got Int instead",
      "expected next token to be Ident, got Assign instead",
      "no prefix parse function for Assign found",
      "expected next token to be Ident, got Int instead",
    ];

    for (i, err) in errors.iter().enumerate() {
      assert_eq!(p.errors[i], err.to_string());
    }
  }

  #[test]
  fn test_identifier_expression() {
    let input = "foobar;";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0].as_ref();

    assert_eq!(stmt.token_literal(), "foobar");
    assert_eq!(stmt.string(), "foobar");
  }

  #[test]
  fn test_intenger_literal_expression() {
    let input = "5;";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0].as_ref();
    assert_eq!(stmt.token_literal(), "5");
    assert_eq!(stmt.string(), "5");
  }

  #[test]
  fn test_parsing_prefix_expressions() {
    let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

    for tt in prefix_tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      assert_eq!(program.statements.len(), 1);

      let stmt = program.statements[0].as_ref();

      assert_eq!(stmt.token_literal(), tt.1);

      assert_eq!(stmt.string(), format!("({}{})", tt.1, tt.2));
    }
  }

  #[test]
  fn test_parsing_prefix_booleans_expressions() {
    let prefix_tests = vec![("!true;", "!", true), ("!false;", "!", false)];

    for tt in prefix_tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      assert_eq!(program.statements.len(), 1);

      let stmt = program.statements[0].as_ref();

      assert_eq!(stmt.token_literal(), tt.1);

      assert_eq!(stmt.string(), format!("({}{})", tt.1, tt.2));
    }
  }

  #[test]
  fn test_parsing_infix_expressions() {
    let infix_tests = vec![
      ("5 + 5;", 5, "+", 5),
      ("5 - 5;", 5, "-", 5),
      ("5 * 5;", 5, "*", 5),
      ("5 / 5;", 5, "/", 5),
      ("5 > 5;", 5, ">", 5),
      ("5 < 5;", 5, "<", 5),
      ("5 == 5;", 5, "==", 5),
      ("5 != 5;", 5, "!=", 5),
    ];

    for tt in infix_tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      assert_eq!(program.statements.len(), 1);

      let stmt = program.statements[0].as_ref();

      assert_eq!(stmt.string(), format!("({} {} {})", tt.1, tt.2, tt.3));
    }
  }

  #[test]
  fn test_parsing_infix_booleans_expressions() {
    let infix_tests = vec![
      ("true == true", true, "==", true),
      ("true != false", true, "!=", false),
      ("false == false", false, "==", false),
    ];

    for tt in infix_tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      assert_eq!(program.statements.len(), 1);

      let stmt = program.statements[0].as_ref();

      assert_eq!(stmt.string(), format!("({} {} {})", tt.1, tt.2, tt.3));
    }
  }

  #[test]
  fn test_operator_precedence_parsing() {
    let tests = vec![
      ("-a * b", "((-a) * b)"),
      ("!-a", "(!(-a))"),
      ("a + b + c", "((a + b) + c)"),
      ("a + b - c", "((a + b) - c)"),
      ("a * b * c", "((a * b) * c)"),
      ("a * b / c", "((a * b) / c)"),
      ("a + b / c", "(a + (b / c))"),
      ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
      ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
      ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
      ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
      (
        "3 + 4 * 5 == 3 * 1 + 4 * 5",
        "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
      ),
      ("true", "true"),
      ("false", "false"),
      ("3 > 5 == false", "((3 > 5) == false)"),
      ("3 < 5 == true", "((3 < 5) == true)"),
      ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
      ("(5 + 5) * 2", "((5 + 5) * 2)"),
      ("2 / (5 + 5)", "(2 / (5 + 5))"),
      ("-(5 + 5)", "(-(5 + 5))"),
      ("!(true == true)", "(!(true == true))"),
    ];

    for tt in tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      if program.statements.len() > 1 {
        let combined_statements = format!(
          "{}{}",
          program.statements[0].string(),
          program.statements[1].string()
        );
        assert_eq!(combined_statements, tt.1);
      } else {
        assert_eq!(program.statements[0].string(), tt.1);
      }
    }
  }

  #[test]
  fn test_boolean_expression() {
    let tests = vec![("true;", true), ("false;", false)];

    for tt in tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      assert_eq!(program.statements.len(), 1);

      let stmt = program.statements[0].as_ref();

      assert_eq!(stmt.token_literal(), tt.1.to_string());
    }
  }

  #[test]
  fn test_if_expression() {
    let input = "if (x < y) { x }";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0].as_ref();

    assert_eq!(stmt.token_literal(), "if");
    assert_eq!(stmt.string(), "if (x < y) x");
  }

  #[test]
  fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0].as_ref();

    assert_eq!(stmt.token_literal(), "if");
    assert_eq!(stmt.string(), "if (x < y) xelse y");
  }

  #[test]
  fn test_function_literal_parsing() {
    let input = "def(x, y) { x + y; }";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);

    let stmt = program.statements[0].as_ref();

    assert_eq!(stmt.token_literal(), "def");
    assert_eq!(stmt.string(), "def(x, y)(x + y)");
  }

  #[test]
  fn test_function_parameter_parsing() {
    let tests = vec![
      ("def() {};", vec![]),
      ("def(x) {};", vec!["x"]),
      ("def(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for tt in tests {
      let l = Lexer::new(tt.0.to_string());
      let mut p = Parser::new(l);
      let program = p.parse_program().unwrap();

      let stmt = program.statements[0].as_ref();

      assert_eq!(stmt.token_literal(), "def");
      if tt.1.is_empty() {
        assert_eq!(stmt.string(), "def()");
      } else {
        assert_eq!(stmt.string(), format!("def({})", tt.1.join(", ")));
      }
    }
  }
}
