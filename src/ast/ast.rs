use crate::token::Token;

pub trait Node {
  fn token_literal(&self) -> String;
}

pub trait Statement: Node {
  fn statement_node(&self);
}

pub trait Expression: Node {
  fn expression_node(&self);
}

pub struct Program {
  pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
  pub fn new() -> Self {
    Self {
      statements: Vec::new(),
    }
  }
}

impl Node for Program {
  fn token_literal(&self) -> String {
    if !self.statements.is_empty() {
      self.statements[0].token_literal()
    } else {
      String::from("")
    }
  }
}

pub struct VarStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<Box<dyn Expression>>,
}

impl Statement for VarStatement {
  fn statement_node(&self) {}
}

impl Node for VarStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
}

pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Expression for Identifier {
  fn expression_node(&self) {}
}

impl Node for Identifier {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }
}

pub struct RetStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for RetStatement {
    fn statement_node(&self) {}
}

impl Node for RetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
