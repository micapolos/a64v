mod a64;
mod asm;
mod jit;
mod op;

use jit::Jit;
use std::mem::transmute;

pub fn main() {
  let mut jit = Jit::new(4096).unwrap();

  let asm = asm! {
    sub_r(x0, x0, x1);
    ret();
  };

  jit.copy_from_slice(&asm);

  jit.set_exec().unwrap();

  type U64Op2 = unsafe extern "C" fn(u64, u64) -> u64;

  let fun: U64Op2 = unsafe { transmute(jit.ptr) };

  let result = unsafe { fun(50, 40) };

  println!("{}", result);
}
