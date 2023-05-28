// Token types constants
const ILLEGAL: &'static str = "ILLEGAL";
const EOF: &'static str = "EOF";
const IDENT: &'static str = "IDENT";
const INT: &'static str = "INT";
const ASSIGN: &'static str = "=";
const PLUS: &'static str = "+";
const COMMA: &'static str = ",";
const SEMICOLON: &'static str = ";";
const LPAREN: &'static str = "(";
const RPAREN: &'static str = ")";
const LBRACE: &'static str = "{";
const RBRACE: &'static str = "}";
const FUNCTION: &'static str = "FUNCTION";
const LET: &'static str = "LET";

struct TokenType(String);

pub struct Token {
    token_type: TokenType,
    literal: String,
}


