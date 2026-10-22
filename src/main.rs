use std::ptr;

mod op;
mod a64;

extern "C" {
  fn sys_icache_invalidate(addr: *mut std::ffi::c_void, len: usize);
}

pub fn main() {
  let size = 4096;

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
    panic!("mmap failed!");
  }

  let code = [
    a64!(add_r(x0, x0, x1)),
    a64!(ret()),
  ];

  unsafe {
    ptr::copy_nonoverlapping(
      code.as_ptr() as *const u8,
      ptr as *mut u8,
      code.len() * std::mem::size_of::<u32>(),
    );
  }

  let protect_res = unsafe {
    libc::mprotect(ptr, size, libc::PROT_READ | libc::PROT_EXEC)
  };

  if protect_res != 0 {
    panic!("mprotect failed to make memory executable!");
  }

  unsafe {
    sys_icache_invalidate(ptr, size);
  }

  type U64Op2 = unsafe extern "C" fn(u64, u64) -> u64;

  let arm_add: U64Op2 = unsafe { std::mem::transmute(ptr) };

  let result = unsafe { arm_add(10, 20) };

  println!("{}", result);

  unsafe {
    libc::munmap(ptr, size);
  }
}
