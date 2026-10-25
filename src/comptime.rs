#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  U32,
  U64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub input: Vec<Type>,
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
      input: vec![Type::U32, Type::U64],
      output: Some(Type::U64),
    };
  }
}
