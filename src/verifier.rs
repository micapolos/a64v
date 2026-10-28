use {crate::function, verifier::Error};

mod verifier {
  #[derive(Debug, PartialEq)]
  pub enum Error {
    AlreadyInFrame,
    MissingFrame,
    InvalidParamCount,
    OutOfMemory,
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct Verifier {
  function_frame_option: Option<Box<function::Frame>>,
}

impl Verifier {
  pub fn fun(&mut self, param_count: usize) -> Result<(), Error> {
    if self.function_frame_option.is_some() {
      return Err(Error::AlreadyInFrame);
    }
    if param_count > 8 {
      return Err(Error::InvalidParamCount);
    }
    self.function_frame_option =
      Some(Box::new(function::Frame::new(param_count)));
    Ok(())
  }

  pub fn ret(&mut self) -> Result<(), Error> {
    let _ = self
      .function_frame_option
      .as_mut()
      .ok_or(Error::MissingFrame)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_verifier_invalid_param_count() {
    let mut verifier = Verifier::default();
    assert_eq!(verifier.fun(10), Err(Error::InvalidParamCount));
  }

  #[test]
  fn test_verifier_fun() {
    let mut verifier = Verifier::default();
    assert_eq!(verifier.fun(2), Ok(()));
  }

  #[test]
  fn test_verifier_fun_ret() {
    let mut verifier = Verifier::default();
    assert_eq!(verifier.fun(2), Ok(()));
    assert_eq!(verifier.ret(), Ok(()));
  }
}
