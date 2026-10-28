#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
  U64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Reg {
  #[default]
  Undefined,
  Defined(Type),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Frame {
  pub param_count: usize,
  pub regs: [Reg; 32],
  pub terminated: bool,
}

impl Frame {
  pub fn new(param_count: usize) -> Self {
    let mut regs = [Reg::Undefined; 32];
    for i in 0..param_count {
      regs[i] = Reg::Defined(Type::U64);
    }
    Self {
      param_count,
      regs,
      terminated: false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_frame_default() {
    let _ = Frame::default();
  }

  #[test]
  fn test_frame() {
    let _ = Frame::new(2);
  }
}
