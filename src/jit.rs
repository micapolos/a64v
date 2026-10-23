use rustix::mm::{mmap_anonymous, mprotect, munmap, MapFlags, MprotectFlags, ProtFlags};
use std::{
  ffi::c_void,
  mem::size_of,
  ptr::null_mut,
};

extern "C" {
  fn sys_icache_invalidate(addr: *mut c_void, len: usize);
}

pub struct Jit {
  pub ptr: *mut u8,
  pub size: usize,
}

impl Jit {
  pub fn new(size: usize) -> Option<Self> {
    let ptr = unsafe {
      mmap_anonymous(
        null_mut(),
        size,
        ProtFlags::READ | ProtFlags::WRITE,
        MapFlags::PRIVATE,
      )
      .ok()?
    };

    Some(Self {
      ptr: ptr as *mut u8,
      size,
    })
  }

  pub fn set_executable(&mut self) -> Option<()> {
    unsafe {
      mprotect(
        self.ptr as *mut c_void,
        self.size,
        MprotectFlags::READ | MprotectFlags::EXEC,
      )
    }
    .ok()?;

    unsafe {
      sys_icache_invalidate(self.ptr as *mut c_void, self.size);
    }

    Some(())
  }

  pub fn as_slice(&mut self) -> &mut [u32] {
    unsafe {
      return std::slice::from_raw_parts_mut(self.ptr as *mut u32, self.size / size_of::<u32>());
    }
  }

  pub fn copy_from_slice(&mut self, slice: &[u32]) {
    self.as_slice()[..slice.len()].copy_from_slice(&slice);
  }
}

impl Drop for Jit {
  fn drop(&mut self) {
    unsafe {
      let _ = munmap(self.ptr as *mut c_void, self.size);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::asm;
  use std::mem::transmute;

  #[test]
  fn test_jit() {
    let asm = asm! {
      sub_r(x0, x0, x1);
      ret();
    };

    let mut jit = Jit::new(4096).unwrap();
    jit.copy_from_slice(&asm);
    jit.set_executable().unwrap();

    type U64Op2 = unsafe extern "C" fn(u64, u64) -> u64;
    let fun: U64Op2 = unsafe { transmute(jit.ptr) };
    let result = unsafe { fun(50, 40) };
    assert_eq!(result, 10);
  }
}
