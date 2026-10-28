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
  param_count: usize,
  regs: [Reg; 32],
  code: Vec<u32>,
}

impl Frame {
  pub fn new() -> Self {
   Frame {
      param_count: 0,
      regs: [Reg::Undefined; 32],
      code: Vec::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_frame() {
    let _ = Frame::new();
  }
}
