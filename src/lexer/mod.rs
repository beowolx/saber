use crate::token::{
  Token, TokenType, ASSIGN, ASTERISK, BANG, COMMA, EOF, EQ, GT, LBRACE, LPAREN,
  LT, MINUS, NOT_EQ, PLUS, RBRACE, RPAREN, SEMICOLON, SLASH,
};

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
      ASSIGN => {
        if self.peek_char() == ASSIGN {
          self.read_char();
          Token::new(TokenType::Eq, EQ.to_owned())
        } else {
          Token::new(TokenType::Assign, ASSIGN.to_string())
        }
      }
      SEMICOLON => Token::new(TokenType::Semicolon, SEMICOLON.to_string()),
      LPAREN => Token::new(TokenType::Lparen, LPAREN.to_string()),
      RPAREN => Token::new(TokenType::Rparen, RPAREN.to_string()),
      COMMA => Token::new(TokenType::Comma, COMMA.to_string()),
      PLUS => Token::new(TokenType::Plus, PLUS.to_string()),
      MINUS => Token::new(TokenType::Minus, MINUS.to_string()),
      BANG => {
        if self.peek_char() == ASSIGN {
          self.read_char();
          Token::new(TokenType::NotEq, NOT_EQ.to_owned())
        } else {
          Token::new(TokenType::Bang, BANG.to_string())
        }
      }
      ASTERISK => Token::new(TokenType::Asterisk, ASTERISK.to_string()),
      SLASH => Token::new(TokenType::Slash, SLASH.to_string()),
      LT => Token::new(TokenType::Lt, LT.to_string()),
      GT => Token::new(TokenType::Gt, GT.to_string()),
      LBRACE => Token::new(TokenType::Lbrace, LBRACE.to_string()),
      RBRACE => Token::new(TokenType::Rbrace, RBRACE.to_string()),
      EOF => Token::new(TokenType::Eof, EOF.to_string()),
      _ => {
        if self.is_letter() {
          let literal = self.read_identifier();
          let token_type = TokenType::lookup_ident(&literal);
          return Token::new(token_type, literal);
        } else if self.ch.is_ascii_digit() {
          let literal = self.read_number();
          return Token::new(TokenType::Int, literal);
        } else {
          Token::new(TokenType::Illegal, self.ch.to_string())
        }
      }
    };
    self.read_char();
    tok
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = EOF;
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

  fn peek_char(&self) -> char {
    if self.read_position >= self.input.len() {
      EOF
    } else {
      self
        .input
        .chars()
        .nth(self.read_position)
        .expect("Peeking char failed")
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
      TokenType::Assign,
      TokenType::Plus,
      TokenType::Lparen,
      TokenType::Rparen,
      TokenType::Lbrace,
      TokenType::Rbrace,
      TokenType::Comma,
      TokenType::Semicolon,
      TokenType::Eof,
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
        forge five_test = 5;
        forge ten = 10;
        forge add = weave(x, y) {
          x + y;
        };

        forge result = add(five, ten);

        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
          ignite true;
        } else {
          ignite false;
        }

        10 == 10;
        10 != 9;
        "
    .to_owned();

    let tests = vec![
      TokenType::Forge,
      TokenType::Ident,
      TokenType::Assign,
      TokenType::Int,
      TokenType::Semicolon,
      TokenType::Forge,
      TokenType::Ident,
      TokenType::Assign,
      TokenType::Int,
      TokenType::Semicolon,
      TokenType::Forge,
      TokenType::Ident,
      TokenType::Assign,
      TokenType::Function,
      TokenType::Lparen,
      TokenType::Ident,
      TokenType::Comma,
      TokenType::Ident,
      TokenType::Rparen,
      TokenType::Lbrace,
      TokenType::Ident,
      TokenType::Plus,
      TokenType::Ident,
      TokenType::Semicolon,
      TokenType::Rbrace,
      TokenType::Semicolon,
      TokenType::Forge,
      TokenType::Ident,
      TokenType::Assign,
      TokenType::Ident,
      TokenType::Lparen,
      TokenType::Ident,
      TokenType::Comma,
      TokenType::Ident,
      TokenType::Rparen,
      TokenType::Semicolon,
      TokenType::Bang,
      TokenType::Minus,
      TokenType::Slash,
      TokenType::Asterisk,
      TokenType::Int,
      TokenType::Semicolon,
      TokenType::Int,
      TokenType::Lt,
      TokenType::Int,
      TokenType::Gt,
      TokenType::Int,
      TokenType::Semicolon,
      TokenType::If,
      TokenType::Lparen,
      TokenType::Int,
      TokenType::Lt,
      TokenType::Int,
      TokenType::Rparen,
      TokenType::Lbrace,
      TokenType::Ignite,
      TokenType::True,
      TokenType::Semicolon,
      TokenType::Rbrace,
      TokenType::Else,
      TokenType::Lbrace,
      TokenType::Ignite,
      TokenType::False,
      TokenType::Semicolon,
      TokenType::Rbrace,
      TokenType::Int,
      TokenType::Eq,
      TokenType::Int,
      TokenType::Semicolon,
      TokenType::Int,
      TokenType::NotEq,
      TokenType::Int,
      TokenType::Semicolon,
      TokenType::Eof,
    ];

    let mut l = Lexer::new(input);
    for t in tests {
      let tok = l.next_token();
      assert_eq!(tok.token_type, t);
    }
  }
}
