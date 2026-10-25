use crate::function::{Id, Offset};

pub struct Function {
  pub id: Id,
  pub offset: Offset,
}

pub struct Value(u64);

impl From<u64> for Value {
  fn from(n: u64) -> Self {
    Value(n)
  }
}

impl TryFrom<Value> for u64 {
  type Error = ();

  fn try_from(value: Value) -> Result<Self, Self::Error> {
    Ok(value.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_value_from() {
    assert_eq!(Value::from(0x1234).try_into(), Ok(0x1234));
  }
}
