#[derive(Debug, PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,
  IDENT,
  INT,
  ASSIGN,
  PLUS,
  COMMA,
  SEMICOLON,
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  FUNCTION,
  VAR,
}

impl TokenType {
  pub fn lookup_ident(ident: &str) -> Self {
    match ident {
      "def" => Self::FUNCTION,
      "var" => Self::VAR,
      _ => Self::IDENT,
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

impl Token {
  pub fn new(token_type: TokenType, literal: String) -> Self {
    Self {
      token_type,
      literal,
    }
  }
}
