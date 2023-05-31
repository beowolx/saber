pub const DEF: &str = "def";
pub const VAR: &str = "var";
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const IF: &str = "if";
pub const ELSE: &str = "else";
pub const RET: &str = "ret";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
pub const ASSIGN: char = '=';
pub const SEMICOLON: char = ';';
pub const LPAREN: char = '(';
pub const RPAREN: char = ')';
pub const LBRACE: char = '{';
pub const RBRACE: char = '}';
pub const COMMA: char = ',';
pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const BANG: char = '!';
pub const ASTERISK: char = '*';
pub const SLASH: char = '/';
pub const LT: char = '<';
pub const GT: char = '>';
pub const EOF: char = '\0';

#[derive(Debug, PartialEq, Clone)]
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
      DEF => Self::Function,
      VAR => Self::Var,
      TRUE => Self::True,
      FALSE => Self::False,
      IF => Self::If,
      ELSE => Self::Else,
      RET => Self::Ret,
      _ => Self::Ident,
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
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
