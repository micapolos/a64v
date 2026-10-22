#[macro_export]
macro_rules! op {
  ($($width:literal $val:expr),+ $(,)?) => {
    {
      let mut acc = 0;
      $(
        let width = $width;
        let val = $val;
        let mask = (1 << width) - 1;
        acc = (acc << width) | (val & mask);
      )*
      acc as u32
    }
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_op() {
    assert_eq!(op!(16 0x1234), 0x1234);
    assert_eq!(op!(3 0b101, 3 0b000, 3 0b010), 0b101000010);
    assert_eq!(op!(4 0b1111, 8 0x12, 16 0x3456), 0xf123456);
  }
}
