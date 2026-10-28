use crate::code;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
  U64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg {
  Undefined,
  Defined(Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
  pub param_count: usize,
  pub regs: [Reg; 32],
  pub code_writer: code::Writer,
}

impl Frame {
  pub fn empty() -> Self {
    Self {
      param_count: 0,
      regs: [Reg::Undefined; 32],
      code_writer: code::Writer::empty(),
    }
  }

  pub fn new(param_count: usize, code_writer: code::Writer) -> Self {
    let mut regs = [Reg::Undefined; 32];
    for i in 0..param_count {
      regs[i] = Reg::Defined(Type::U64);
    }
    Self {
      param_count,
      regs,
      code_writer,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_frame() {
    let mut code = [0u32, 16];
    let _ = Frame::new(2, code::Writer::new(code.as_mut_ptr(), 16));
  }
}
