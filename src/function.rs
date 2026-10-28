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
  pub code: Vec<u32>,
}

impl Frame {
  pub fn new(param_count: usize) -> Self {
    let mut regs = [Reg::Undefined; 32];
    for i in 0..param_count {
      regs[i] = Reg::Defined(Type::U64);
    }
    Self { param_count, regs, code: Vec::new() }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_frame() {
    let _ = Frame::new(2);
  }
}
