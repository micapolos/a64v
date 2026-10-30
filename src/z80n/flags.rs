pub const CARRY_BIT: u8 = 0;
pub const SUBTRACT_BIT: u8 = 1;
pub const PARITY_OVERFLOW_BIT: u8 = 2;
pub const HALF_CARRY_BIT: u8 = 4;
pub const ZERO_BIT: u8 = 6;
pub const SIGN_BIT: u8 = 7;

pub const CARRY_MASK: u8 = 1 << CARRY_BIT;
pub const SUBTRACT_MASK: u8 = 1 << SUBTRACT_BIT;
pub const PARITY_OVERFLOW_MASK: u8 = 1 << PARITY_OVERFLOW_BIT;
pub const HALF_CARRY_MASK: u8 = 1 << HALF_CARRY_BIT;
pub const ZERO_MASK: u8 = 1 << ZERO_BIT;
pub const SIGN_MASK: u8 = 1 << SIGN_BIT;
