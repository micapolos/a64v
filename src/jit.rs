use std::io;
use std::ptr;

extern "C" {
  fn sys_icache_invalidate(addr: *mut std::ffi::c_void, len: usize);
}

pub struct Jit {
  pub ptr: *mut u8,
  pub size: usize,
}

impl Jit {
  pub fn new(size: usize) -> io::Result<Self> {
    let ptr = unsafe {
      libc::mmap(
        ptr::null_mut(),
        size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_ANON | libc::MAP_PRIVATE,
        -1,
        0,
      )
    };

    if ptr == libc::MAP_FAILED {
      return Err(io::Error::last_os_error());
    }

    Ok(Self {
      ptr: ptr as *mut u8,
      size,
    })
  }

  pub fn set_exec(&mut self) -> io::Result<()> {
    let res = unsafe {
      libc::mprotect(
        self.ptr as *mut libc::c_void,
        self.size,
        libc::PROT_READ | libc::PROT_EXEC,
      )
    };

    if res != 0 {
      return Err(io::Error::last_os_error());
    }

    unsafe {
      sys_icache_invalidate(self.ptr as *mut libc::c_void, self.size);
    }

    Ok(())
  }

  pub fn as_slice(&mut self) -> &mut [u32] {
    unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut u32, self.size) }
  }

  pub fn copy_from_slice(&mut self, slice: &[u32]) {
    self.as_slice()[..slice.len()].copy_from_slice(&slice);
  }
}

impl Drop for Jit {
  fn drop(&mut self) {
    unsafe {
      libc::munmap(self.ptr as *mut libc::c_void, self.size);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_jit() {
    let asm = crate::asm! {
      sub_r(x0, x0, x1)
      ret()
    };

    let mut jit = Jit::new(4096).unwrap();
    jit.copy_from_slice(&asm);
    jit.set_exec().unwrap();

    type U64Op2 = unsafe extern "C" fn(u64, u64) -> u64;
    let fun: U64Op2 = unsafe { std::mem::transmute(jit.ptr) };
    let result = unsafe { fun(50, 40) };
    assert_eq!(result, 10);
  }
}
