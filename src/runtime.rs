use {crate::comptime::Type, std::mem::transmute};

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
  typ: Type,
  payload: usize,
}

impl Value {
  pub fn new_u64(v: u64) -> Self {
    Value {
      typ: Type::U64,
      payload: v as usize,
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

    let arg0 = if let Some(v) = args.get(0) {
      v.payload
    } else {
      0
    };
    let arg1 = if let Some(v) = args.get(1) {
      v.payload
    } else {
      0
    };
    let arg2 = if let Some(v) = args.get(2) {
      v.payload
    } else {
      0
    };
    let arg3 = if let Some(v) = args.get(3) {
      v.payload
    } else {
      0
    };
    let arg4 = if let Some(v) = args.get(4) {
      v.payload
    } else {
      0
    };
    let arg5 = if let Some(v) = args.get(5) {
      v.payload
    } else {
      0
    };
    let arg6 = if let Some(v) = args.get(6) {
      v.payload
    } else {
      0
    };
    let arg7 = if let Some(v) = args.get(7) {
      v.payload
    } else {
      0
    };

    type Fn = extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let fun: Fn = unsafe { transmute(self.payload) };
    let result = fun(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);

    Ok(match &func.output {
      Some(result_type) => Some(Value {
        typ: result_type.clone(),
        payload: result,
      }),
      None => None,
    })
  }
}

#[cfg(test)]
mod tests {
  use {super::*, crate::comptime::Function};

  extern "C" fn c_sub(x: u64, y: u64) -> u64 {
    x - y
  }

  #[test]
  fn test_value_call() {
    let function = Function {
      input: Box::new([Type::U64, Type::U64]),
      output: Some(Type::U64),
    };

    let typ = Type::Function(Box::new(function));

    let value = Value {
      typ,
      payload: c_sub as *const () as usize,
    };

    let result = value.call(vec![Value::new_u64(30), Value::new_u64(20)]).unwrap();

    assert_eq!(result, Some(Value::new_u64(10)));
  }
}
