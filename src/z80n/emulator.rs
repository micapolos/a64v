use crate::bits;

pub struct R(u8);

impl R {
  pub fn decode(index: u8) -> Self {
    R(index & 0b111)
  }

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

pub struct Regs(u64);

impl Regs {
  pub fn new() -> Self {
    Regs(0)
  }

  pub fn get(&self, r: R) -> u8 {
    let shift = Self::shift(r);
    ((self.0 >> shift) & 0xff) as u8
  }

  pub fn set(&mut self, r: R, n: u8) {
    let shift = Self::shift(r);
    let mask = 0xff << shift;
    self.0 = (self.0 & !mask) | ((n as u64) << shift);
  }

  #[inline]
  fn shift(r: R) -> usize {
    r.index() << 3
  }
}

pub struct Z80N {
  pub regs: Regs,
  pub pc: u16,
  pub sp: u16,
}

impl Z80N {
  pub fn new() -> Self {
    Z80N {
      regs: Regs::new(),
      pc: 0,
      sp: 0,
    }
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
  pub fn new(io: IOImpl) -> Self {
    Emulator {
      z80n: Z80N::new(),
      io,
    }
  }

  pub fn step(&mut self) {
    let op = self.fetch();
    match bits!(op, 3, 3) {
      0b01 => {
        self.ld_r_r(R::decode(op >> 3), R::decode(op));
      }
      0b10 => {
        match bits!(op, 3) {
          0b000 => {
            self.add_r(R::decode(op));
          }
          _ => {}
        }
      }
      0b11 => {
        match bits!(op, 6) {
          0b001001 => {
            self.ret();
          }
          _ => {}
        }
      }
      _ => {}
    }
  }

  fn fetch(&mut self) -> u8 {
    let op = self.io.read(self.z80n.pc);
    self.z80n.pc += 1;
    op
  }

  fn ld_r_r(&mut self, r1: R, r2: R) {
    let regs = &mut self.z80n.regs;
    regs.set(r1, regs.get(r2));
  }

  fn add_r(&mut self, r: R) {
    let regs = &mut self.z80n.regs;
    regs.set(R::A, regs.get(R::A) + regs.get(r));
  }

  fn ret(&mut self) {
    // TODO
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
    let _ = Regs::new();
  }

  #[test]
  fn test_z80n() {
    let _ = Z80N::new();
  }

  #[test]
  fn test_emulator() {
    let mem = [0_u8; 0x10000];
    let mut emulator = Emulator::new(mem);
    emulator.step();
    emulator.step();
    assert_eq!(emulator.z80n.pc, 2);
  }
}
