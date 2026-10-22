#[macro_export]
macro_rules! reg {
  (x0) => (0);
  (x1) => (1);
  (x2) => (2);
  (x3) => (3);
  (x4) => (4);
  (x5) => (5);
  (x6) => (6);
  (x7) => (7);
  (x30) => (30);
}

#[macro_export]
macro_rules! a64 {
  (nop) => (0b11010101000000110010000000011111_u32);
  (ret_r $reg:tt) => ($crate::op!(22 0b1101011001011111000000, 5 reg!($reg), 5 0b00000));
  (ret) => (a64!(ret_r x30));
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_reg() {
    assert_eq!(reg!(x30), 30);
  }

  #[test]
  fn test_a64() {
    assert!(a64!(nop) != 0);
    assert!(a64!(ret_r x30) != 0);
    assert_eq!(a64!(ret), a64!(ret_r x30));
  }
}
