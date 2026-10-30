use crate::z80n::reg::Regs;

pub struct Cpu {
  pub regs: Regs,
  pub pc: u16,
  pub sp: u16,
  pub is_halted: bool,
}

impl Cpu {
  #[inline]
  pub fn new() -> Self {
    Cpu {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_z80n() {
    let _ = Cpu::new();
  }
}
