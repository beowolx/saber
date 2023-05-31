use std::env;
mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
  let user = env::var("USER").expect("Failed to get the current user");

  println!(
    "💧 Hello {}! Welcome to the Mizu programming language!",
    user
  );
  println!("Feel free to type in commands");
  repl::Repl::start();
}
