#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
  index: u8,
}

impl Register {
  pub fn with_index(index: u32) -> Self {
    assert!(index < 32);
    Self { index: index as u8 }
  }

  pub fn index(&self) -> u32 {
    self.index as u32
  }

  pub fn mask(&self) -> u32 {
    1 << self.index()
  }

  pub const X0: Register = Self { index: 0 };
  pub const X1: Register = Self { index: 1 };
  pub const X2: Register = Self { index: 2 };
  pub const X3: Register = Self { index: 3 };
  pub const XZR: Register = Self { index: 31 };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Set {
  mask: u32,
}

impl Set {
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
  fn test_register_with_index() {
    assert_eq!(Register::with_index(0), Register::X0);
    assert_eq!(Register::with_index(31), Register::XZR);
  }

  #[test]
  fn test_register_index() {
    assert_eq!(Register::X0.index(), 0);
    assert_eq!(Register::X1.index(), 1);
    assert_eq!(Register::XZR.index(), 31);
  }

  #[test]
  fn test_register_mask() {
    assert_eq!(Register::X0.mask(), 0b1);
    assert_eq!(Register::X1.mask(), 0b10);
    assert_eq!(Register::XZR.mask(), 0b10000000_00000000_00000000_00000000);
  }

  #[test]
  fn test_set_empty() {
    let mut set = Set::empty();
    assert!(set.is_empty());

    assert!(!set.contains(Register::X0));
    assert!(!set.contains(Register::X1));
    assert!(!set.contains(Register::XZR));

    set.add(Register::X0);
    set.add(Register::XZR);

    assert!(set.contains(Register::X0));
    assert!(!set.contains(Register::X1));
    assert!(set.contains(Register::XZR));
  }
}
