#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bit {
  Zero = 0,
  One = 1,
}

#[macro_export]
macro_rules! bit {
  ($expr:expr, $n:expr) => {
    unsafe { core::mem::transmute::<u8, Bit>((($expr >> $n) & 1) as u8) }
  };
}
