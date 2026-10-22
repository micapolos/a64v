#![no_std]
#![no_main]

#[link(name = "System")]
extern "C" {
    fn write(fd: i32, cbuf: *const u8, nbyte: usize) -> isize;
    fn _exit(status: i32) -> !;
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
  unsafe {
    // File descriptor 1 is stdout
    write(1, b"!\n".as_ptr(), 2);
    _exit(0);
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  unsafe {
    _exit(1);
  }
}
