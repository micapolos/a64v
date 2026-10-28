use {crate::function, assembler::Error};
use crate::jit::Jit;

mod assembler {
  #[derive(Debug, PartialEq)]
  pub enum Error {
    AlreadyInFrame,
    MissingFrame,
    InvalidParamCount,
  }
}

pub struct Assembler {
  jit: Jit,
  function_frame_option: Option<Box<function::Frame>>,
}

impl Assembler {
  pub fn new() -> Result<Self, ()> {
    let jit = Jit::new(0x10000).ok_or(())?;
    Ok(Assembler {
      jit,
      function_frame_option: None,
    })
  }

  pub fn fun(&mut self, param_count: usize) -> Result<(), Error> {
    if self.function_frame_option.is_some() {
      return Err(Error::AlreadyInFrame);
    }
    if param_count > 8 {
      return Err(Error::InvalidParamCount);
    }
    self.function_frame_option = Some(Box::new(function::Frame::new(param_count)));
    Ok(())
  }

  pub fn ret(&mut self) -> Result<(), Error> {
    let frame = self
      .function_frame_option
      .as_mut()
      .ok_or(Error::MissingFrame)?;
    frame.code.push(123);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_assembler_invalid_param_count() {
    let mut assembler = Assembler::new().unwrap();
    assert_eq!(assembler.fun(10), Err(Error::InvalidParamCount));
  }

  #[test]
  fn test_assembler_fun() {
    let mut assembler = Assembler::new().unwrap();
    assert_eq!(assembler.fun(2), Ok(()));
  }

  #[test]
  fn test_assembler_fun_ret() {
    let mut assembler = Assembler::new().unwrap();
    assert_eq!(assembler.fun(2), Ok(()));
    assert_eq!(assembler.ret(), Ok(()));
  }
}
