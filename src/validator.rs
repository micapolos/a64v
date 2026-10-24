pub struct Validator {
  mut pc: u64;
}

pub struct Entry {
  pub mut input_registers_mask: RegisterSet;
  pub mut output_registers_mask: RegisterSet;
  pub mut clobbered_registers_mask: RegisterSet;
}

pub struct Frame {
  pub mut defined_registers_mask: RegisterSet;
  pub mut dirty_registers_mask: RegisterSet;
}

impl Frame {
  fn is_register_readable(register: Register) -> Bool {
    defined_registers_mask & register.mask() != 0
  }
}

impl Validator {
  pub fn emit_a64(op: u32) {
    match bits!(op, 5, 24) {
      // add
      0b01011 => {

      }
      _ => panic!();
    }
  }

  pub fn emit_v64(a64: u32) {

  }
}
