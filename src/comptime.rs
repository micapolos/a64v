#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  U32,
  U64,
  Function(Box<Function>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub input: Box<[Type]>,
  pub output: Option<Type>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_type() {
    assert_eq!(Type::U64, Type::U64);
  }

  #[test]
  fn test_function() {
    let _ = Function {
      input: Box::new([Type::U32, Type::U64]),
      output: Some(Type::U64),
    };
  }
}
