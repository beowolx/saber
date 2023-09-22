#[derive(Debug)]
pub enum ObjectType {
  Integer(i64),
  Boolean,
  DarkSide,
}

trait Object {
  fn object_type(&self) -> ObjectType;
  fn inspect(&self) -> String;
}

struct Integer {
  value: i64,
}

impl Object for Integer {
  fn inspect(&self) -> String {
    self.value.to_string()
  }
  fn object_type(&self) -> ObjectType {
    ObjectType::Integer(self.value)
  }
}

struct Boolean {
  value: bool,
}

impl Object for Boolean {
  fn inspect(&self) -> String {
    self.value.to_string()
  }
  fn object_type(&self) -> ObjectType {
    ObjectType::Boolean
  }
}

struct DarkSide;

impl Object for DarkSide {
  fn inspect(&self) -> String {
    "DarkSide".to_owned()
  }
  fn object_type(&self) -> ObjectType {
    ObjectType::DarkSide
  }
}
