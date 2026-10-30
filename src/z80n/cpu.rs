use crate::z80n::reg::Regs;

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
  fn read(&self, _address: u16) -> u8 {
    0
  }
  fn write(&mut self, _address: u16, _value: u8) {}
  fn input(&self, _address: u16) -> u8 {
    0
  }
  fn output(&mut self, _address: u16, _value: u8) {}
  fn halt(&mut self, _z80n: &mut Z80N) {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_z80n() {
    let _ = Z80N::new();
  }
}
