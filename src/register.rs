pub struct Register {
  index: u8,
}

impl Register {
  pub fn index(&self) -> u32 {
    self.index as u32
  }

  pub const X0: Register = Self { index: 0 };
  pub const X1: Register = Self { index: 1 };
  pub const X2: Register = Self { index: 1 };
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
}
