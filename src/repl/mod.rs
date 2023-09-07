use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;
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

      let lexer = Lexer::new(buffer);
      let mut parser = Parser::new(lexer);

      let program = parser.parse_program().unwrap();
      if !parser.errors.is_empty() {
        Repl::print_parser_errors(parser.errors);
        continue;
      }

      println!("{}", program.string());
    }
  }
  fn print_parser_errors(errors: Vec<String>) {
    println!("Woops! 🌊 Something went wrong 🌊");
    println!("エラーが発生しました！(An error occurred!)"); // Japanese Localization
    println!(" parser errors:");

    for msg in errors {
      println!("\t{}", msg);
    }
  }
}
