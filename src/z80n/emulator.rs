pub struct GeneralRegisters {
  pub bcdehlfa: u64,
}

impl GeneralRegisters {
  pub fn new() -> Self {
    GeneralRegisters { bcdehlfa: 0 }
  }
}

pub struct Z80N {
  pub general_registers: GeneralRegisters,
  pub pc: u16,
  pub sp: u16,
}

impl Z80N {
  pub fn new() -> Self {
    Z80N {
      general_registers: GeneralRegisters::new(),
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
    let _ = self.fetch();
  }

  fn fetch(&mut self) -> u8 {
    let op = self.io.read(self.z80n.pc);
    self.z80n.pc += 1;
    op
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
  fn test_general_registers() {
    let _ = GeneralRegisters::new();
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
