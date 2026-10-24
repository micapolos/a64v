use crate::register_set::RegisterSet;
use crate::register::Register;

pub struct Frame {
  defined_register_set: RegisterSet,
  readable_register_set: RegisterSet,
  writable_register_set: RegisterSet,
  dirty_register_set: RegisterSet,
}

impl Frame {
  pub fn new() -> Self {
    Frame {
      defined_register_set: RegisterSet::empty(),
      readable_register_set: RegisterSet::empty(),
      writable_register_set: RegisterSet::empty(),
      dirty_register_set: RegisterSet::empty(),
    }
  }

  pub fn read_register(&self, register: Register) -> Result<(), &'static str> {
    if !self.readable_register_set.contains(register) {
      return Err("register not readable");
    }
    if !self.defined_register_set.contains(register) {
      return Err("register undefined");
    }
    Ok(())
  }

  pub fn write_register(&mut self, register: Register) -> Result<(), &'static str> {
    if !self.writable_register_set.contains(register) {
      return Err("register not writable");
    }
    self.dirty_register_set.add(register);
    self.defined_register_set.add(register);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_frame() {
    let mut frame = Frame::new();
    frame.defined_register_set.add(Register::X0);
    frame.readable_register_set.add(Register::X0);
    frame.writable_register_set.add(Register::X0);
    frame.read_register(Register::X0).expect("readable");
    assert!(!frame.dirty_register_set.contains(Register::X0));
    frame.write_register(Register::X0).expect("writable");
    assert!(frame.dirty_register_set.contains(Register::X0));
  }
}
