pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";
pub const IDENT: &'static str = "IDENT";
pub const INT: &'static str = "INT";
pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";
pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";
pub const LPAREN: &'static str = "(";
pub const RPAREN: &'static str = ")";
pub const LBRACE: &'static str = "{";
pub const RBRACE: &'static str = "}";
pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";

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
  LET,
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
