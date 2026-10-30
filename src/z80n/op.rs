#[derive(Clone, Copy, PartialEq)]
pub struct Op(u8);

impl Op {
  pub fn u8(&self) -> u8 {
    self.0
  }

  pub fn nop() -> Self {
    Op(0)
  }

  pub fn halt() -> Self {
    Op(0b01_110_110)
  }

  pub fn ld_r_n(r: R) -> Self {
    Op((0b00 << 6) | (r.0 << 3) | 0b110)
  }

  pub fn ld_r_r(r1: R, r2: R) -> Self {
    Op((0b01 << 6) | (r1.0 << 3) | r2.0)
  }

  pub fn ret() -> Self {
    Op((0b11 << 6) | 0b001001)
  }

  pub fn rst(n: u8) -> Self {
    Op((0b11 << 6) | (n & 0b111000) | 0b111)
  }
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
