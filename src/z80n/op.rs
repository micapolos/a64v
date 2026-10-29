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

#[derive(Clone, Copy, PartialEq)]
pub struct R(u8);

impl R {
  #[inline]
  pub fn decode(op: u8) -> Self {
    Self(op & 0b111)
  }

  #[inline]
  pub fn u8(&self) -> u8 {
    self.0
  }

  pub const B: Self = Self(0);
  pub const C: Self = Self(1);
  pub const D: Self = Self(2);
  pub const E: Self = Self(3);
  pub const H: Self = Self(4);
  pub const L: Self = Self(5);
  pub const IMM_HL: Self = Self(6);
  pub const A: Self = Self(7);
}

#[derive(Clone, Copy, PartialEq)]
pub struct Alu(u8);

impl Alu {
  #[inline]
  pub fn decode(op: u8) -> Self {
    Self(op & 0b111)
  }

  #[inline]
  pub fn u8(&self) -> u8 {
    self.0
  }

  pub const ADD: Self = Self(0);
  pub const ADC: Self = Self(1);
  pub const SUB: Self = Self(2);
  pub const SBC: Self = Self(3);
  pub const AND: Self = Self(4);
  pub const OR: Self = Self(5);
  pub const XOR: Self = Self(6);
  pub const CP: Self = Self(7);
}

