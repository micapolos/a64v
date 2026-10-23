#[macro_export]
macro_rules! asm {
  ($( $op:ident $( $arg:tt ),* )*) => {
    [
      $(
        $crate::a64!($op $( $arg ),*)
      ),*
    ]
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_asm() {
    let code = asm! {
      add_r(x0, x0, x1)
      add_r(x0, x0, x1)
      ret()
    };
    println!("{}", code[0]);
  }
}
