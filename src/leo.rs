#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Begin(String),
  Symbol(String),
  Number(u64),
  End,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_token() {
    let _ = Token::Begin("foo".into());
    let _ = Token::Symbol("foo".into());
    let _ = Token::Number(123);
    let _ = Token::End;
  }
}

pub mod emitter;
pub mod token;
