use {crate::comptime::Type, std::mem::transmute};

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
  typ: Type,
  x: usize,
}

impl From<u64> for Value {
  fn from(x: u64) -> Self {
    Value {
      typ: Type::U64,
      x: x as usize,
    }
  }
}

impl Value {
  pub fn call(&self, args: Vec<Value>) -> Result<Option<Value>, &'static str> {
    let Type::Function(ref func) = self.typ else {
      return Err("not a function");
    };

    if args.len() != func.input.len() {
      return Err("arg count mismatch");
    }

    if args.len() > 8 {
      return Err("max args 8");
    }

    let mut xs = [0usize; 8];
    for (dest, src) in xs.iter_mut().zip(&args) {
      *dest = src.x;
    }

    type Fn = extern "C" fn(
      usize,
      usize,
      usize,
      usize,
      usize,
      usize,
      usize,
      usize,
    ) -> usize;
    let fun: Fn = unsafe { transmute(self.x) };
    let result = fun(xs[0], xs[1], xs[2], xs[3], xs[4], xs[5], xs[6], xs[7]);

    Ok(match &func.output {
      Some(result_type) => Some(Value {
        typ: result_type.clone(),
        x: result,
      }),
      None => None,
    })
  }
}

#[cfg(test)]
mod tests {
  use {super::*, crate::comptime::Function};

  #[test]
  fn test_value_call_void() {
    extern "C" fn c_void() {}

    let function = Function {
      input: Box::new([]),
      output: None,
    };

    let typ = Type::Function(Box::new(function));

    let value = Value {
      typ,
      x: c_void as *const () as usize,
    };

    let result = value.call(vec![]);

    assert_eq!(result, Ok(None));
  }

  #[test]
  fn test_value_call_sub() {
    extern "C" fn c_sub(x: u64, y: u64) -> u64 {
      x - y
    }

    let function = Function {
      input: Box::new([Type::U64, Type::U64]),
      output: Some(Type::U64),
    };

    let typ = Type::Function(Box::new(function));

    let value = Value {
      typ,
      x: c_sub as *const () as usize,
    };

    let result = value.call(vec![Value::from(30), Value::from(20)]);

    assert_eq!(result, Ok(Some(Value::from(10))));
  }

  #[test]
  fn test_value_call_not_function() {
    let value = Value::from(0);
    let result = value.call(vec![]);
    assert_eq!(result, Err("not a function"));
  }

  #[test]
  fn test_value_call_arg_count_mismatch() {
    let function = Function {
      input: Box::new([Type::U64, Type::U64]),
      output: Some(Type::U64),
    };

    let typ = Type::Function(Box::new(function));

    let value = Value { typ, x: 0 };

    let result = value.call(vec![Value::from(30)]);

    assert_eq!(result, Err("arg count mismatch"));
  }

  #[test]
  fn test_value_call_max_args() {
    let function = Function {
      input: Box::new([
        Type::U64,
        Type::U64,
        Type::U64,
        Type::U64,
        Type::U64,
        Type::U64,
        Type::U64,
        Type::U64,
        Type::U64,
      ]),
      output: Some(Type::U64),
    };

    let typ = Type::Function(Box::new(function));

    let value = Value { typ, x: 0 };

    let result = value.call(vec![
      Value::from(30),
      Value::from(30),
      Value::from(30),
      Value::from(30),
      Value::from(30),
      Value::from(30),
      Value::from(30),
      Value::from(30),
      Value::from(30),
    ]);

    assert_eq!(result, Err("max args 8"));
  }
}
