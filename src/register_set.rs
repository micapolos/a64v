use crate::register::Register;

pub struct RegisterSet {
  mask: u32,
}

impl RegisterSet {
  pub fn empty() -> Self {
    Self { mask: 0 }
  }

  pub fn is_empty(&self) -> bool {
    self.mask == 0
  }

  pub fn contains(&self, register: Register) -> bool {
    self.mask & register.index() != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    let register_set = RegisterSet::empty();
    assert!(!register_set.contains(Register::X0));
    assert!(register_set.is_empty());
  }
}
