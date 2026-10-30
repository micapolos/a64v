use crate::z80n::op;

#[derive(Clone, Copy)]
pub struct R(u8);

impl R {
  #[inline]
  pub fn decode(r: op::R) -> Self {
    R(r.u8())
  }

  #[inline]
  pub fn index(&self) -> usize {
    self.0 as usize
  }

  pub const B: R = R(0);
  pub const C: R = R(1);
  pub const D: R = R(2);
  pub const E: R = R(3);
  pub const H: R = R(4);
  pub const L: R = R(5);
  pub const F: R = R(6);
  pub const A: R = R(7);
}

#[derive(Clone, Copy)]
pub struct RR(u8);

impl RR {
  #[inline]
  pub fn decode(index: u8) -> Self {
    RR(index & 0b11)
  }

  #[inline]
  pub fn index(&self) -> usize {
    self.0 as usize
  }

  pub const BC: RR = RR(0);
  pub const DE: RR = RR(1);
  pub const HL: RR = RR(2);
  pub const AF: RR = RR(3);
}

pub struct Regs(u64);

impl Regs {
  #[inline]
  pub fn new() -> Self {
    Regs(0)
  }

  #[inline]
  pub fn get_r(&self, r: R) -> u8 {
    let shift = Self::shift_r(r);
    ((self.0 >> shift) & 0xff) as u8
  }

  #[inline]
  pub fn get_rr(&self, rr: RR) -> u16 {
    let shift = Self::shift_rr(rr);
    ((self.0 >> shift) & 0xffff) as u16
  }

  #[inline]
  pub fn set_r(&mut self, r: R, n: u8) {
    let shift = Self::shift_r(r);
    self.0 = (self.0 & !(0xff << shift)) | ((n as u64) << shift);
  }

  #[inline]
  pub fn set_rr(&mut self, rr: RR, nn: u16) {
    let shift = Self::shift_rr(rr);
    self.0 = (self.0 & !(0xffff << shift)) | ((nn as u64) << shift);
  }

  #[inline]
  pub fn inc_r(&mut self, r: R) {
    self.set_r(r, self.get_r(r).wrapping_add(1));
  }

  #[inline]
  pub fn dec_r(&mut self, r: R) {
    self.set_r(r, self.get_r(r).wrapping_sub(1));
  }

  #[inline]
  pub fn inc_rr(&mut self, rr: RR) {
    self.set_rr(rr, self.get_rr(rr).wrapping_add(1));
  }

  #[inline]
  pub fn dec_rr(&mut self, rr: RR) {
    self.set_rr(rr, self.get_rr(rr).wrapping_sub(1));
  }

  #[inline]
  pub fn shift_r(r: R) -> usize {
    r.index() << 3
  }

  #[inline]
  pub fn shift_rr(rr: RR) -> usize {
    rr.index() << 4
  }
}
