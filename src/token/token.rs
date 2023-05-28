#[derive(Debug, PartialEq)]
pub enum TokenType {
  Illegal,
  Eof,
  Ident,
  Int,
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  Lt,
  Gt,
  Comma,
  Semicolon,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Function,
  Var,
  True,
  False,
  If,
  Else,
  Ret,
  Eq,
  NotEq,
}

impl TokenType {
  pub fn lookup_ident(ident: &str) -> Self {
    match ident {
      "def" => Self::Function,
      "var" => Self::Var,
      "true" => Self::True,
      "false" => Self::False,
      "if" => Self::If,
      "else" => Self::Else,
      "ret" => Self::Ret,
      _ => Self::Ident,
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
