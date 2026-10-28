#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Writer {
  pub ptr: *mut u32,
  pub left: usize,
}

impl Writer {
  pub fn empty() -> Self {
    Writer {
      ptr: 0 as *mut u32,
      left: 0,
    }
  }

  pub fn new(ptr: *mut u32, left: usize) -> Self {
    Writer { ptr, left }
  }

  pub fn write(&mut self, op: u32) -> Result<(), ()> {
    if self.left == 0 {
      return Err(());
    }

    unsafe {
      self.ptr.write(op);
      self.ptr = self.ptr.add(1);
    }
    self.left -= 1;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_writer() {
    let mut arr = [0, 0, 0, 0];
    let mut writer = Writer {
      ptr: arr.as_mut_ptr(),
      left: 2,
    };
    assert_eq!(writer.write(10), Ok(()));
    assert_eq!(writer.write(20), Ok(()));
    assert_eq!(writer.write(30), Err(()));
    assert_eq!(arr, [10, 20, 0, 0]);
  }
}
