use crate::register::Set;

pub struct Function {
  pub input_register_set: Set,
  pub output_register_set: Set,
  pub stack_size: usize,
}

impl Function {
  pub fn empty() -> Self {
    Self {
      input_register_set: Set::empty(),
      output_register_set: Set::empty(),
      stack_size: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let function = Function::empty();
    assert_eq!(function.stack_size, 0);
  }
}
