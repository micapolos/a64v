use {crate::function, crate::function::{Reg, Type}, verifier::Error};

mod verifier {
  use super::*;

  #[derive(Debug, PartialEq)]
  pub enum Error {
    AlreadyInFrame,
    MissingFrame,
    InvalidParamCount,
    OutOfMemory,
    RegisterNotWritable(Register),
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Register { index: u8, }

impl Register {
  pub fn new(index: usize) -> Result<Self, ()> {
    if index & !0b11111 != 0 {
      return Err(());
    }
    Ok(Register { index: (index & 0b11111) as u8 })
  }

  pub const R0: Register = Register { index: 0 };
  pub const R1: Register = Register { index: 1 };
  pub const R2: Register = Register { index: 2 };
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

  fn verify_register_writable(&self, register: Register) -> Result<(), Error> {
    if register.index > 18 {
      return Err(Error::RegisterNotWritable(register));
    }
    Ok(())
  }

  pub fn movz(&mut self, register: Register, typ: Type) -> Result<(), Error> {
    self.verify_register_writable(register)?;
    let function_state = self
      .function_frame_option
      .as_mut()
      .ok_or(Error::MissingFrame)?;
    function_state.regs[register.index as usize] = Reg::Defined(typ);
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
