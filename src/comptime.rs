#[derive(Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Type {
  U64,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_type() {
    assert_eq!(Type::U64, Type::U64);
  }
}
