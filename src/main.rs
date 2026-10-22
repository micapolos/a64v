mod op;
mod a64;

pub fn main() {
  println!("0x{:08X}", op!(16 1, 16 2));
  println!("0x{:08X}", a64!(nop));
}
