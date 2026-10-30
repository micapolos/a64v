use crate::{
  bit,
  bit::Bit,
  bits,
  z80n::{
    cpu::Cpu,
    op,
    reg::{R, RR},
    value::{h, hl, l},
  },
};

pub trait IO {
  fn read(&self, _address: u16) -> u8 {
    0
  }
  fn write(&mut self, _address: u16, _value: u8) {}
  fn input(&self, _address: u16) -> u8 {
    0
  }
  fn output(&mut self, _address: u16, _value: u8) {}
  fn halt(&mut self, _z80n: &mut Cpu) {}
}

pub struct Emulator<IOImpl: IO> {
  pub cpu: Cpu,
  pub io: IOImpl,
}

impl<IOImpl: IO> Emulator<IOImpl> {
  #[inline]
  pub fn new(io: IOImpl) -> Self {
    Emulator {
      cpu: Cpu::new(),
      io,
    }
  }

  #[inline]
  pub fn step(&mut self) {
    if self.cpu.is_halted {
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
    while !self.cpu.is_halted {
      self.step();
    }
  }

  #[inline]
  pub fn fetch(&mut self) -> u8 {
    let op = self.io.read(self.cpu.pc);
    // std::thread::sleep(std::time::Duration::from_millis(500));
    // println!("Fetched 0x{:04X}: 0x{:02X}", self.cpu.pc, op);
    self.cpu.inc_pc();
    op
  }

  #[inline]
  pub fn get_r(&self, r: op::R) -> u8 {
    match r {
      op::R::IMM_HL => self.io.read(self.cpu.regs.get_rr(RR::HL)),
      _ => self.cpu.regs.get_r(R::decode(r)),
    }
  }

  #[inline]
  pub fn set_r(&mut self, r: op::R, n: u8) {
    match r {
      op::R::IMM_HL => {
        self.io.write(self.cpu.regs.get_rr(RR::HL), n);
      }
      _ => {
        self.cpu.regs.set_r(R::decode(r), n);
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
    let mut a = self.cpu.regs.get_r(R::A);
    a = a.wrapping_add(n);
    self.cpu.regs.set_r(R::A, a);
  }

  #[inline]
  pub fn sub_r(&mut self, r: op::R) {
    let n = self.get_r(r);
    let mut a = self.cpu.regs.get_r(R::A);
    a = a.wrapping_sub(n);
    self.cpu.regs.set_r(R::A, a);
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
    self.cpu.dec_sp();
    self.io.write(self.cpu.sp, h(nn));
    self.cpu.dec_sp();
    self.io.write(self.cpu.sp, l(nn));
  }

  #[inline]
  pub fn pop_nn(&mut self) -> u16 {
    let l = self.io.read(self.cpu.sp);
    self.cpu.inc_sp();
    let h = self.io.read(self.cpu.sp);
    self.cpu.inc_sp();
    hl(h, l)
  }

  #[inline]
  pub fn ret(&mut self) {
    self.cpu.pc = self.pop_nn();
  }

  #[inline]
  pub fn rst(&mut self, n: u8) {
    self.cpu.pc = n as u16;
  }

  #[inline]
  pub fn nop(&mut self) {}

  #[inline]
  pub fn halt(&mut self) {
    self.cpu.is_halted = true;
    self.io.halt(&mut self.cpu);
  }
}

#[cfg(test)]
mod tests {
  use {super::*, crate::z80n::op::Op};

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

    fn halt(&mut self, z80n: &mut Cpu) {
      if z80n.pc == 9 {
        self.did_call_hook = true;
        z80n.is_halted = false;
      }
    }
  }

  #[test]
  fn test_emulator_halt() {
    let mut emulator = Emulator::new(TestIO::new());
    emulator.io.mem[0x100] = Op::halt().u8();
    emulator.cpu.pc = 0x100;
    emulator.run();
    assert_eq!(emulator.io.did_call_hook, false);
    assert_eq!(emulator.cpu.pc, 0x101);
  }

  #[test]
  fn test_emulator_hook() {
    let mut emulator = Emulator::new(TestIO::new());
    emulator.io.mem[0x8] = Op::halt().u8();
    emulator.io.mem[0x9] = Op::halt().u8();
    emulator.cpu.pc = 0x0a;
    emulator.run();
    assert_eq!(emulator.io.did_call_hook, true);
    assert_eq!(emulator.cpu.pc, 0x0a);
  }

  #[test]
  fn test_emulator_add() {
    let mut emulator = Emulator::new(TestIO::new());
    emulator.io.mem[0x100] = Op::ld_r_n(op::R::A).u8();
    emulator.io.mem[0x101] = 0x12;
    emulator.io.mem[0x102] = Op::halt().u8();
    emulator.cpu.pc = 0x100;
    emulator.run();
    assert_eq!(emulator.cpu.pc, 0x103);
    assert_eq!(emulator.cpu.regs.get_r(R::A), 0x12);
  }
}
