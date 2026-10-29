#[derive(Clone, Copy, PartialEq)]
pub struct Op(u8);

#[derive(Clone, Copy, PartialEq)]
pub struct Category(u8);

impl Category {
  #[inline]
  pub fn decode(op: u8) -> Self {
    Self(op & 0b11)
  }

  pub const MISC: Self = Self(0b00);
  pub const LOAD: Self = Self(0b01);
  pub const ALU: Self = Self(0b10);
  pub const CONTROL: Self = Self(0b11);
}
