use crate::register::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

  pub fn add(&mut self, register: Register) {
    self.mask = (self.mask & !register.mask()) | register.mask();
  }

  pub fn contains(&self, register: Register) -> bool {
    self.mask & register.mask() != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    let mut register_set = RegisterSet::empty();
    assert!(register_set.is_empty());

    assert!(!register_set.contains(Register::X0));
    assert!(!register_set.contains(Register::X1));
    assert!(!register_set.contains(Register::XZR));

    register_set.add(Register::X0);
    register_set.add(Register::XZR);

    assert!(register_set.contains(Register::X0));
    assert!(!register_set.contains(Register::X1));
    assert!(register_set.contains(Register::XZR));
  }
}
