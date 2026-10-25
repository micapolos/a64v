use crate::{bits, register::Register};

#[macro_export]
macro_rules! reg {
  (x0) => {
    0
  };
  (x1) => {
    1
  };
  (x2) => {
    2
  };
  (x3) => {
    3
  };
  (x4) => {
    4
  };
  (x5) => {
    5
  };
  (x6) => {
    6
  };
  (x7) => {
    7
  };
  (x8) => {
    8
  };
  (x9) => {
    9
  };
  (x10) => {
    10
  };
  (x11) => {
    11
  };
  (x12) => {
    12
  };
  (x13) => {
    13
  };
  (x14) => {
    14
  };
  (x15) => {
    15
  };
  (x30) => {
    30
  };
}

#[macro_export]
macro_rules! a64 {
  (nop()) => {
    0b11010101000000110010000000011111_u32
  };
  (ret()) => {
    $crate::a64!(ret_r(x30))
  };
  (ret_r($reg:tt)) => {
    $crate::op!(
      22 0b1101011001011111000000,
      5 $crate::reg!($reg),
      5 0b00000,
    )
  };
  (add_r($r1:tt, $r2:tt, $r3:tt)) => {
    $crate::op!(
      11 0b10001011000,
      5 $crate::reg!($r3),
      6 0,
      5 $crate::reg!($r2),
      5 $crate::reg!($r1),
    )
  };
  (sub_r($r1:tt, $r2:tt, $r3:tt)) => {
    $crate::op!(
      11 0b11001011000,
      5 $crate::reg!($r3),
      6 0,
      5 $crate::reg!($r2),
      5 $crate::reg!($r1),
    )
  };
  (movz($r:tt, $imm16:expr)) => {
    $crate::op!(
      1 0b1,
      2 0b10,
      6 0b100101,
      2 0b00,
      16 $imm16,
      5 $crate::reg!($r),
    )
  };
}

pub fn is_ari_sh_reg(op: u32) -> bool {
  bits!(op, 5, 24) == 0b01011
}

pub fn ari_sh_reg_sf(op: u32) -> u32 {
  bits!(op, 1, 31)
}

pub fn ari_sh_reg_op(op: u32) -> u32 {
  bits!(op, 1, 30)
}

pub fn ari_sh_reg_s(op: u32) -> u32 {
  bits!(op, 1, 29)
}

pub fn ari_sh_reg_rm(op: u32) -> u32 {
  bits!(op, 5, 16)
}

pub fn ari_sh_reg_imm6(op: u32) -> u32 {
  bits!(op, 6, 10)
}

pub fn ari_sh_reg_rn(op: u32) -> u32 {
  bits!(op, 5, 5)
}

pub fn ari_sh_reg_rd(op: u32) -> u32 {
  bits!(op, 5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
  MoveZeroImmediateWide {
    destination_register: Register,
    immediate: u16,
  },
  AddRegister {
    destination_register: Register,
    left_register: Register,
    right_register: Register,
  },
  Return {
    address_register: Register,
  },
}

impl Instruction {
  pub fn decode(op: u32) -> Option<Self> {
    if op & 0b111_11111_111_00000_111111_00000_00000
      == 0b100_01011_000_00000_000000_00000_00000
    {
      Some(Instruction::AddRegister {
        destination_register: Register::decode(op),
        left_register: Register::decode(op >> 5),
        right_register: Register::decode(op >> 16),
      })
    } else if op & 0b111_111111_11_0000000000000000_00000
      == 0b110_100101_00_0000000000000000_00000
    {
      Some(Instruction::MoveZeroImmediateWide {
        destination_register: Register::decode(op),
        immediate: bits!(op, 16, 5) as u16,
      })
    } else if op & 0b111_111_11111_11111_1111_11_00000_11111
      == 0b110_101_10010_11111_0000_00_00000_00000
    {
      Some(Instruction::Return {
        address_register: Register::decode(bits!(op, 5, 5)),
      })
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reg() {
    assert_eq!(reg!(x30), 30);
  }

  #[test]
  fn test_a64() {
    assert!(a64!(nop()) != 0);
    assert!(a64!(ret_r(x30)) != 0);
    assert_eq!(a64!(ret()), a64!(ret_r(x30)));
    assert_ne!(a64!(add_r(x0, x0, x1)), 0);
  }

  #[test]
  fn test_ari_sh_reg() {
    assert!(is_ari_sh_reg(0b100_01011_00_0_00000_000000_00000_00000));
    assert_eq!(ari_sh_reg_sf(0b000_01011_00_0_00000_000000_00000_00000), 0);
    assert_eq!(ari_sh_reg_sf(0b100_01011_00_0_00000_000000_00000_00000), 1);
    assert_eq!(ari_sh_reg_op(0b000_01011_00_0_00000_000000_00000_00000), 0);
    assert_eq!(ari_sh_reg_op(0b010_01011_00_0_00000_000000_00000_00000), 1);
    assert_eq!(ari_sh_reg_s(0b000_01011_00_0_00000_000000_00000_00000), 0);
    assert_eq!(ari_sh_reg_s(0b001_01011_00_0_00000_000000_00000_00000), 1);
    assert_eq!(
      ari_sh_reg_rm(0b000_01011_00_0_10100_000000_00000_00000),
      0b10100
    );
    assert_eq!(
      ari_sh_reg_imm6(0b000_01011_00_0_00000_100110_00000_00000),
      0b100110
    );
    assert_eq!(
      ari_sh_reg_rn(0b000_01011_00_0_00000_000000_10100_00000),
      0b10100
    );
    assert_eq!(
      ari_sh_reg_rd(0b000_01011_00_0_00110_000000_00000_10100),
      0b10100
    );
  }

  #[test]
  fn test_decode() {
    assert_eq!(
      Instruction::decode(a64!(ret_r(x2))),
      Some(Instruction::Return {
        address_register: Register::X2,
      })
    );

    assert_eq!(
      Instruction::decode(a64!(add_r(x1, x2, x3))),
      Some(Instruction::AddRegister {
        destination_register: Register::X1,
        left_register: Register::X2,
        right_register: Register::X3,
      })
    );

    assert_eq!(
      Instruction::decode(a64!(movz(x1, 0x1234))),
      Some(Instruction::MoveZeroImmediateWide {
        destination_register: Register::X1,
        immediate: 0x1234,
      })
    );
  }
}
