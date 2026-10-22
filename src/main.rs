#![no_std]
#![no_main]

#[link(name = "System")]
extern "C" {
    fn write(fd: i32, cbuf: *const u8, nbyte: usize) -> isize;
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
  unsafe {
    write(1, b"!\n".as_ptr(), 2);
    0
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  unsafe {
    core::arch::asm!("brk 0", options(noreturn));
  }
}
