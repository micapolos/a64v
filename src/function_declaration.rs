use crate::register_set::RegisterSet;

pub struct FunctionDeclaration {
  pub input_registers: RegisterSet,
  pub output_registers: RegisterSet,
  pub stack_size: usize,
}

impl FunctionDeclaration {
  pub fn empty() -> Self {
    Self {
      input_registers: RegisterSet::empty(),
      output_registers: RegisterSet::empty(),
      stack_size: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let function_declaration = FunctionDeclaration::empty();
    assert_eq!(function_declaration.stack_size, 0);
  }
}
