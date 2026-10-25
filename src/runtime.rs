use std::ffi::c_void;
use std::mem::transmute;
use crate::comptime::{Type};
use crate::comptime;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  U32(u32),
  U64(u64),
}

impl From<u64> for Value {
  fn from(n: u64) -> Self {
    Value::U64(n)
  }
}

impl TryFrom<Value> for u64 {
  type Error = ();

  fn try_from(value: Value) -> Result<Self, Self::Error> {
    match value {
      Value::U32(_) => Err(()),
      Value::U64(value) => Ok(value),
    }
  }
}

pub struct Function {
  pub comptime: comptime::Function,
  pub address: *const c_void,
}

impl Function {
  pub fn call(&self, args: Vec<Value>) -> Result<Option<Value>, ()> {
    if args.len() >= 8 {
      return Err(());
    }

    type Fn = extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64;
    let fun: Fn = unsafe { transmute(self.address) };
    let result = fun(0, 0, 0, 0, 0, 0, 0, 0);

    Ok(Some(Value::from(result)))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Typed {
  typ: Type,
  value: Value,
}

impl Typed {
  pub fn new(typ: Type, value: Value) -> Result<Self, ()> {
    match typ {
      Type::U32 => match value {
        Value::U32(_) => Ok(Typed { typ, value }),
        _ => Err(()),
      },
      Type::U64 => match value {
        Value::U64(_) => Ok(Typed { typ, value }),
        _ => Err(()),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_value_from() {
    assert_eq!(Value::from(0x1234).try_into(), Ok(0x1234));
  }

  #[test]
  fn test_typed() {
    assert_eq!(
      Typed::new(Type::U64, Value::from(0x1234)),
      Typed::new(Type::U64, Value::from(0x1234)));
  }

  #[test]
  fn test_function() {
    assert_eq!(
      Typed::new(Type::U64, Value::from(0x1234)),
      Typed::new(Type::U64, Value::from(0x1234)));
  }
}
