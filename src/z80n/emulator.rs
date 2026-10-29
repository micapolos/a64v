use crate::bits;

#[inline]
pub fn rr(hi: u8, lo: u8) -> u16 {
  (hi as u16) << 8 | (lo as u16)
}

#[inline]
pub fn lo(rr: u16) -> u8 {
  rr as u8
}

#[inline]
pub fn hi(rr: u16) -> u8 {
  lo(rr >> 8)
}

#[derive(Clone, Copy)]
pub struct R(u8);

impl R {
  #[inline]
  pub fn decode(index: u8) -> Self {
    R(index & 0b111)
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

pub struct Z80N {
  pub regs: Regs,
  pub pc: u16,
  pub sp: u16,
  pub is_halted: bool,
}

impl Z80N {
  #[inline]
  pub fn new() -> Self {
    Z80N {
      regs: Regs::new(),
      pc: 0,
      sp: 0,
      is_halted: false,
    }
  }

  #[inline]
  pub fn inc_pc(&mut self) {
    self.pc = self.pc.wrapping_add(1);
  }

  #[inline]
  pub fn inc_sp(&mut self) {
    self.sp = self.sp.wrapping_add(1);
  }

  #[inline]
  pub fn dec_sp(&mut self) {
    self.sp = self.sp.wrapping_sub(1);
  }
}

pub trait IO {
  fn read(&self, _address: u16) -> u8 { 0 }
  fn write(&mut self, _address: u16, _value: u8) {}
  fn input(&self, _address: u16) -> u8 { 0 }
  fn output(&mut self, _address: u16, _value: u8) {}
}

pub struct Emulator<IOImpl: IO> {
  pub z80n: Z80N,
  pub io: IOImpl,
}

impl<IOImpl: IO> Emulator<IOImpl> {
  #[inline]
  pub fn new(io: IOImpl) -> Self {
    Emulator {
      z80n: Z80N::new(),
      io,
    }
  }

  #[inline]
  pub fn step(&mut self) {
    if self.z80n.is_halted {
      return;
    }

    let op = self.fetch();
    match bits!(op, 7..6) {
      0b00 => {
        match bits!(op, 5..0) {
          0b000000 => {
            self.nop();
          }
          _ => {}
        }
      }
      0b01 => {
        let r1 = bits!(op, 5..3);
        let r2 = bits!(op, 2..0);
        if r1 == 0b110 && r2 == 0b110 {
          self.halt();
        } else {
          self.ld_r_r(R::decode(r1), R::decode(r2));
        }
      }
      0b10 => {
        match bits!(op, 2..0) {
          0b000 => {
            self.add_r(R::decode(op));
          }
          0b010 => {
            self.sub_r(R::decode(op));
          }
          _ => {}
        }
      }
      0b11 => {
        match bits!(op, 5..0) {
          0b001001 => {
            self.ret();
          }
          _ => {}
        }
      }
      _ => {}
    }
  }

  pub fn run(&mut self, mut max_steps: usize) {
    while max_steps != 0 {
      self.step();
      if self.z80n.is_halted {
        return;
      }
      max_steps -= 1;
    }
  }

  #[inline]
  pub fn fetch(&mut self) -> u8 {
    let op = self.io.read(self.z80n.pc);
    self.z80n.inc_pc();
    op
  }

  #[inline]
  pub fn ld_r_r(&mut self, r1: R, r2: R) {
    let regs = &mut self.z80n.regs;
    regs.set_r(r1, regs.get_r(r2));
  }

  #[inline]
  pub fn add_r(&mut self, r: R) {
    let regs = &mut self.z80n.regs;
    regs.set_r(R::A, regs.get_r(R::A).wrapping_add(regs.get_r(r)));
  }

  #[inline]
  pub fn sub_r(&mut self, r: R) {
    let regs = &mut self.z80n.regs;
    regs.set_r(R::A, regs.get_r(R::A).wrapping_sub(regs.get_r(r)));
  }

  #[inline]
  pub fn push_nn(&mut self, nn: u16) {
    self.z80n.dec_sp();
    self.io.write(self.z80n.sp, hi(nn));
    self.z80n.dec_sp();
    self.io.write(self.z80n.sp, lo(nn));
  }

  #[inline]
  pub fn pop_nn(&mut self) -> u16 {
    let lo = self.io.read(self.z80n.sp);
    self.z80n.inc_sp();
    let hi = self.io.read(self.z80n.sp);
    self.z80n.inc_sp();
    rr(hi, lo)
  }

  #[inline]
  pub fn ret(&mut self) {
    self.z80n.pc = self.pop_nn();
  }

  #[inline]
  pub fn nop(&mut self) {}

  #[inline]
  pub fn halt(&mut self) {
    self.z80n.is_halted = true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rr() {
    assert_eq!(rr(0x12, 0x34), 0x1234);
  }

  #[test]
  fn test_lo() {
    assert_eq!(lo(0x1234), 0x34);
  }

  #[test]
  fn test_hi() {
    assert_eq!(hi(0x1234), 0x12);
  }

  impl IO for [u8; 0x10000] {
    fn read(&self, address: u16) -> u8 {
      self[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
      self[address as usize] = value;
    }
  }

  #[test]
  fn test_regs() {
    let mut regs = Regs::new();
    assert_eq!(regs.get_r(R::B), 0);
    assert_eq!(regs.get_r(R::C), 0);
    assert_eq!(regs.get_r(R::D), 0);
    assert_eq!(regs.get_r(R::E), 0);
    assert_eq!(regs.get_r(R::H), 0);
    assert_eq!(regs.get_r(R::L), 0);
    assert_eq!(regs.get_r(R::F), 0);
    assert_eq!(regs.get_r(R::A), 0);

    assert_eq!(regs.get_rr(RR::BC), 0);
    assert_eq!(regs.get_rr(RR::DE), 0);
    assert_eq!(regs.get_rr(RR::HL), 0);
    assert_eq!(regs.get_rr(RR::AF), 0);

    regs.set_r(R::B, 0x01);
    regs.set_r(R::C, 0x23);
    regs.set_r(R::D, 0x45);
    regs.set_r(R::E, 0x67);
    regs.set_r(R::H, 0x89);
    regs.set_r(R::L, 0xab);
    regs.set_r(R::F, 0xcd);
    regs.set_r(R::A, 0xef);

    assert_eq!(regs.get_r(R::B), 0x01);
    assert_eq!(regs.get_r(R::C), 0x23);
    assert_eq!(regs.get_r(R::D), 0x45);
    assert_eq!(regs.get_r(R::E), 0x67);
    assert_eq!(regs.get_r(R::H), 0x89);
    assert_eq!(regs.get_r(R::L), 0xab);
    assert_eq!(regs.get_r(R::F), 0xcd);
    assert_eq!(regs.get_r(R::A), 0xef);
  }

  #[test]
  fn test_z80n() {
    let _ = Z80N::new();
  }

  #[test]
  fn test_emulator() {
    let mut emulator = Emulator::new([0_u8; 0x10000]);
    emulator.run(123);
    assert_eq!(emulator.z80n.pc, 123);
  }
}
