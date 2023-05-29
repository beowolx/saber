use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io;
use std::io::Write;

const PROMPT: &str = ">> ";

pub struct Repl;

impl Repl {
  pub fn start() {
    loop {
      print!("{}", PROMPT);
      io::stdout().flush().unwrap();

      let mut buffer = String::new();
      io::stdin().read_line(&mut buffer).unwrap();

      let mut lexer = Lexer::new(buffer);
      loop {
        let token = lexer.next_token();
        if token.token_type == TokenType::Eof {
          break;
        }
        println!("{:?}", token);
      }
    }
  }
}
