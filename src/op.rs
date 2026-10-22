#[macro_export]
macro_rules! op {
  ($($width:literal $val:expr),*) => {
    {
      let mut acc: u32 = 0;
      $(
        let width = $width;
        let val = $val;
        let mask = (1 << width) - 1;
        acc = (acc << width) | (val & mask);
      )*
      acc
    }
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_basic_packing() {
    let result = op!(4 0b1010, 4 0b0101);
    assert_eq!(result, 0b10100101);
  }
}
