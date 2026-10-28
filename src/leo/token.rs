use crate::leo::Token;

pub trait Emitter {
  type Error;
  fn emit(&mut self, token: &Token) -> Result<(), Self::Error>;
}
