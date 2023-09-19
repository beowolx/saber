enum ObjectType {
  Integer,
  Boolean,
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
    ObjectType::Integer
  }
}
