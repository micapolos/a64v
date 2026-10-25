#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub input: Box<[Type]>,
  pub output: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  U32,
  U64,
  Function(Box<Function>),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_function() {
    let _ = Function {
      input: Box::new([Type::U32, Type::U64]),
      output: Some(Type::U64),
    };
  }

  #[test]
  fn test_type() {
    let _ = Type::U32;
    let _ = Type::U64;
    let _ = Type::Function(Box::new(Function {
      input: Box::new([Type::U32, Type::U64]),
      output: Some(Type::U64),
    }));
  }
}
