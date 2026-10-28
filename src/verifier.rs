use {
  crate::{code, function, jit::Jit},
  verifier::Error,
};

mod verifier {
  #[derive(Debug, PartialEq)]
  pub enum Error {
    AlreadyInFrame,
    MissingFrame,
    InvalidParamCount,
    OutOfMemory,
  }
}

pub struct Verifier {
  jit: Jit,
  function_frame_option: Option<Box<function::Frame>>,
}

impl Verifier {
  pub fn new() -> Result<Self, ()> {
    let jit = Jit::new(0x10000).ok_or(())?;
    Ok(Verifier {
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
    self.function_frame_option = Some(Box::new(function::Frame::new(
      param_count,
      code::Writer {
        ptr: self.jit.ptr as *mut u32,
        left: self.jit.size >> 2,
      },
    )));
    Ok(())
  }

  pub fn ret(&mut self) -> Result<(), Error> {
    let frame = self
      .function_frame_option
      .as_mut()
      .ok_or(Error::MissingFrame)?;
    frame
      .code_writer
      .write(123)
      .map_err(|_| Error::OutOfMemory)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_verifier_invalid_param_count() {
    let mut verifier = Verifier::new().unwrap();
    assert_eq!(verifier.fun(10), Err(Error::InvalidParamCount));
  }

  #[test]
  fn test_verifier_fun() {
    let mut verifier = Verifier::new().unwrap();
    assert_eq!(verifier.fun(2), Ok(()));
  }

  #[test]
  fn test_verifier_fun_ret() {
    let mut verifier = Verifier::new().unwrap();
    assert_eq!(verifier.fun(2), Ok(()));
    assert_eq!(verifier.ret(), Ok(()));
  }
}
