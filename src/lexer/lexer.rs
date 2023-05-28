use crate::token::{Token, TokenType};

/// The `Lexer` struct is responsible for the lexical analysis of the source code. It breaks down the source code into a sequence of tokens.
///
/// # Fields
///
/// * `input` - The source code to be tokenized.
///
/// * `position` - The current position in the `input` (points to the current character).
///
/// * `read_position` - The current reading position in the `input` (points to the character after the current character).
///
/// * `ch` - The current character under examination.
pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize,
  ch: char,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let mut l = Self {
      input,
      position: 0,
      read_position: 0,
      ch: '\0',
    };
    l.read_char();
    l
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    let tok = match self.ch {
      '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()),
      ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
      '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
      ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
      ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
      '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
      '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
      '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
      '\0' => Token::new(TokenType::EOF, "".to_string()),
      _ => {
        if self.is_letter() {
          let literal = self.read_identifier();
          let token_type = TokenType::lookup_ident(&literal);
          return Token::new(token_type, literal);
        } else if self.ch.is_ascii_digit() {
          let literal = self.read_number();
          return Token::new(TokenType::INT, literal);
        } else {
          Token::new(TokenType::ILLEGAL, self.ch.to_string())
        }
      }
    };
    self.read_char();
    tok
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = '\0';
    } else {
      self.ch = self
        .input
        .chars()
        .nth(self.read_position)
        .expect("Reading char failed");
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn read_identifier(&mut self) -> String {
    let position = self.position;
    while self.is_letter() {
      self.read_char();
    }
    self.input[position..self.position].to_string()
  }

  fn read_number(&mut self) -> String {
    let position = self.position;
    while self.ch.is_ascii_digit() {
      self.read_char();
    }
    self.input[position..self.position].to_string()
  }

  fn is_letter(&self) -> bool {
    self.ch.is_alphabetic() || self.ch == '_'
  }

  fn skip_whitespace(&mut self) {
    while self.ch.is_whitespace() {
      self.read_char();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::lexer::Lexer;
  use crate::token::TokenType;

  #[test]
  fn test_next_token() {
    let input = "=+(){},;".to_owned();
    let tests = vec![
      TokenType::ASSIGN,
      TokenType::PLUS,
      TokenType::LPAREN,
      TokenType::RPAREN,
      TokenType::LBRACE,
      TokenType::RBRACE,
      TokenType::COMMA,
      TokenType::SEMICOLON,
      TokenType::EOF,
    ];
    let mut l = Lexer::new(input);
    for t in tests {
      let tok = l.next_token();
      assert_eq!(tok.token_type, t);
    }
  }

  #[test]
  fn test_lexer() {
    let input = "
        var five_test = 5;
        var ten = 10;
        var add = def(x, y) {
          x + y;
        };
        var result = add(five, ten);
        "
    .to_owned();

    let tests = vec![
      TokenType::VAR,
      TokenType::IDENT,
      TokenType::ASSIGN,
      TokenType::INT,
      TokenType::SEMICOLON,
      TokenType::VAR,
      TokenType::IDENT,
      TokenType::ASSIGN,
      TokenType::INT,
      TokenType::SEMICOLON,
      TokenType::VAR,
      TokenType::IDENT,
      TokenType::ASSIGN,
      TokenType::FUNCTION,
      TokenType::LPAREN,
      TokenType::IDENT,
      TokenType::COMMA,
      TokenType::IDENT,
      TokenType::RPAREN,
      TokenType::LBRACE,
      TokenType::IDENT,
      TokenType::PLUS,
      TokenType::IDENT,
      TokenType::SEMICOLON,
      TokenType::RBRACE,
      TokenType::SEMICOLON,
      TokenType::VAR,
      TokenType::IDENT,
      TokenType::ASSIGN,
      TokenType::IDENT,
      TokenType::LPAREN,
      TokenType::IDENT,
      TokenType::COMMA,
      TokenType::IDENT,
      TokenType::RPAREN,
      TokenType::SEMICOLON,
      TokenType::EOF,
    ];

    let mut l = Lexer::new(input);
    for t in tests {
      let tok = l.next_token();
      assert_eq!(tok.token_type, t);
    }
  }
}
