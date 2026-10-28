use crate::leo::Token;
use crate::leo::token::Emitter;

pub struct Print {
  _private: (),
}

impl Print {
  pub fn new() -> Self {
    Print { _private: () }
  }
}

impl Emitter for Print {
  type Error = ();

  fn emit(&mut self, token: &Token) -> Result<(), ()> {
    match token {
      Token::Begin(s) => {
        print!("( {}", s);
        Ok(())
      },
      Token::Symbol(s) => {
        print!("{} ", s);
        Ok(())
      },
      Token::Number(u) => {
        print!("{} ", u);
        Ok(())
      },
      Token::End => {
        print!(") ");
        Ok(())
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_print() {
    let mut print = Print::new();
    print.emit(&Token::Begin("foo".into())).unwrap();
    print.emit(&Token::End).unwrap();
  }
}
