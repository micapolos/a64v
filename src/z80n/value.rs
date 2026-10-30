#[inline]
pub fn hl(h: u8, l: u8) -> u16 {
  (h as u16) << 8 | (l as u16)
}

#[inline]
pub fn l(hl: u16) -> u8 {
  hl as u8
}

#[inline]
pub fn h(hl: u16) -> u8 {
  l(hl >> 8)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hl() {
    assert_eq!(hl(0x12, 0x34), 0x1234);
  }

  #[test]
  fn test_l() {
    assert_eq!(l(0x1234), 0x34);
  }

  #[test]
  fn test_h() {
    assert_eq!(h(0x1234), 0x12);
  }
}
