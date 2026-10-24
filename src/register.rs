pub struct Register {
  index: u8,
}

impl Register {
  pub fn index(&self) -> u32 {
    self.index as u32
  }

  pub fn mask(&self) -> u32 {
    1 << self.index()
  }

  pub const X0: Register = Self { index: 0 };
  pub const X1: Register = Self { index: 1 };
  pub const X2: Register = Self { index: 2 };
  pub const XZR: Register = Self { index: 31 };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_index() {
    assert_eq!(Register::X0.index(), 0);
    assert_eq!(Register::X1.index(), 1);
    assert_eq!(Register::XZR.index(), 31);
  }

  #[test]
  fn test_mask() {
    assert_eq!(Register::X0.mask(), 0b1);
    assert_eq!(Register::X1.mask(), 0b10);
    assert_eq!(Register::XZR.mask(), 0b10000000_00000000_00000000_00000000);
  }
}
