use crate::bits;

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
}
