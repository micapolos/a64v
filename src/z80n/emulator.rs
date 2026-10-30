use crate::{
  bit,
  bit::Bit,
  bits,
  z80n::{
    value::{hl, l, h},
    cpu::{IO, Z80N},
    op,
    reg::{R, RR},
  },
};

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
    match bit!(op, 7) {
      Bit::Zero => match bit!(op, 6) {
        Bit::Zero => {
          let r = op::R::decode(op >> 3);
          match bits!(op, 2..0) {
            0b100 => {
              self.inc_r(r);
            }
            0b101 => {
              self.dec_r(r);
            }
            0b110 => {
              let n = self.fetch();
              self.set_r(r, n);
            }
            _ => {}
          }
        }
        Bit::One => {
          let r1 = op::R::decode(op >> 3);
          let r2 = op::R::decode(op);
          if r1 == op::R::IMM_HL && r2 == op::R::IMM_HL {
            self.halt();
          } else {
            self.set_r(r1, self.get_r(r2));
          }
        }
      },
      Bit::One => match bit!(op, 6) {
        Bit::Zero => {
          let alu = op::Alu::decode(op >> 3);
          let r = op::R::decode(op);
          match alu {
            op::Alu::ADD => {
              self.add_r(r);
            }
            op::Alu::SUB => {
              self.sub_r(r);
            }
            _ => {}
          }
        }
        Bit::One => match bits!(op, 2..0) {
          0b111 => {
            self.rst(op & 0b111000);
          }
          0b001 => match bits!(op, 5..3) {
            0b001 => {
              self.ret();
            }
            _ => {}
          },
          _ => {}
        },
      },
    }
  }

  pub fn run(&mut self) {
    while !self.z80n.is_halted {
      self.step();
    }
  }

  #[inline]
  pub fn fetch(&mut self) -> u8 {
    let op = self.io.read(self.z80n.pc);
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // println!("Fetched 0x{:04X}: 0x{:02X}", self.z80n.pc, op);
    self.z80n.inc_pc();
    op
  }

  #[inline]
  pub fn get_r(&self, r: op::R) -> u8 {
    match r {
      op::R::IMM_HL => self.io.read(self.z80n.regs.get_rr(RR::HL)),
      _ => self.z80n.regs.get_r(R::decode(r)),
    }
  }

  #[inline]
  pub fn set_r(&mut self, r: op::R, n: u8) {
    match r {
      op::R::IMM_HL => {
        self.io.write(self.z80n.regs.get_rr(RR::HL), n);
      }
      _ => {
        self.z80n.regs.set_r(R::decode(r), n);
      }
    }
  }

  #[inline]
  pub fn ld_r_r(&mut self, r1: op::R, r2: op::R) {
    self.set_r(r1, self.get_r(r2));
  }

  #[inline]
  pub fn add_r(&mut self, r: op::R) {
    let n = self.get_r(r);
    let mut a = self.z80n.regs.get_r(R::A);
    a = a.wrapping_add(n);
    self.z80n.regs.set_r(R::A, a);
  }

  #[inline]
  pub fn sub_r(&mut self, r: op::R) {
    let n = self.get_r(r);
    let mut a = self.z80n.regs.get_r(R::A);
    a = a.wrapping_sub(n);
    self.z80n.regs.set_r(R::A, a);
  }

  #[inline]
  pub fn inc_r(&mut self, r: op::R) {
    self.set_r(r, self.get_r(r).wrapping_add(1));
  }

  #[inline]
  pub fn dec_r(&mut self, r: op::R) {
    self.set_r(r, self.get_r(r).wrapping_sub(1));
  }

  #[inline]
  pub fn push_nn(&mut self, nn: u16) {
    self.z80n.dec_sp();
    self.io.write(self.z80n.sp, h(nn));
    self.z80n.dec_sp();
    self.io.write(self.z80n.sp, l(nn));
  }

  #[inline]
  pub fn pop_nn(&mut self) -> u16 {
    let l = self.io.read(self.z80n.sp);
    self.z80n.inc_sp();
    let h = self.io.read(self.z80n.sp);
    self.z80n.inc_sp();
    hl(h, l)
  }

  #[inline]
  pub fn ret(&mut self) {
    self.z80n.pc = self.pop_nn();
  }

  #[inline]
  pub fn rst(&mut self, n: u8) {
    self.z80n.pc = n as u16;
  }

  #[inline]
  pub fn nop(&mut self) {}

  #[inline]
  pub fn halt(&mut self) {
    self.z80n.is_halted = true;
    self.io.halt(&mut self.z80n);
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    crate::z80n::{op::Op, reg::Regs},
  };

  struct TestIO {
    pub mem: [u8; 0x10000],
    pub did_call_hook: bool,
  }

  impl TestIO {
    fn new() -> Self {
      TestIO {
        mem: [0; 0x10000],
        did_call_hook: false,
      }
    }
  }

  impl IO for TestIO {
    fn read(&self, address: u16) -> u8 {
      self.mem[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
      self.mem[address as usize] = value;
    }

    fn halt(&mut self, z80n: &mut Z80N) {
      if z80n.pc == 9 {
        self.did_call_hook = true;
        z80n.is_halted = false;
      }
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
  fn test_emulator_halt() {
    let mut emulator = Emulator::new(TestIO::new());
    emulator.io.mem[0x100] = Op::halt().u8();
    emulator.z80n.pc = 0x100;
    emulator.run();
    assert_eq!(emulator.io.did_call_hook, false);
    assert_eq!(emulator.z80n.pc, 0x101);
  }

  #[test]
  fn test_emulator_hook() {
    let mut emulator = Emulator::new(TestIO::new());
    emulator.io.mem[0x8] = Op::halt().u8();
    emulator.io.mem[0x9] = Op::halt().u8();
    emulator.z80n.pc = 0x0a;
    emulator.run();
    assert_eq!(emulator.io.did_call_hook, true);
    assert_eq!(emulator.z80n.pc, 0x0a);
  }

  #[test]
  fn test_emulator_add() {
    let mut emulator = Emulator::new(TestIO::new());
    emulator.io.mem[0x100] = Op::ld_r_n(op::R::A).u8();
    emulator.io.mem[0x101] = 0x12;
    emulator.io.mem[0x102] = Op::halt().u8();
    emulator.z80n.pc = 0x100;
    emulator.run();
    assert_eq!(emulator.z80n.pc, 0x103);
    assert_eq!(emulator.z80n.regs.get_r(R::A), 0x12);
  }
}
