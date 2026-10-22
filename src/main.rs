mod op;

pub fn main() {
  println!("0x{:08X}", op!(16 1, 16 2));
}
