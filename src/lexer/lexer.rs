use crate::token::{Token, TokenType};

pub struct Lexer {
  input: String,
  position: usize, // current position in input (points to current char)
  read_position: usize, // current reading position in input (after current char)
  ch: char,             // current char under examination
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
    let tok = match self.ch {
      '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()),
      ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
      '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
      ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
      ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
      '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
      '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
      '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
      '\0' => Token::new(TokenType::EOF, "".to_owned()),
      _ => Token::new(TokenType::ILLEGAL, self.ch.to_string()),
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
}

#[cfg(test)]
mod tests {
  use crate::token::TokenType;

  use super::super::lexer::Lexer;
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
}
