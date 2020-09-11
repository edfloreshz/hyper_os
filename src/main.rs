#![no_std]
#![no_main]
#![feature(global_asm)]

use core::ptr;
mod panic;

const UART0: *mut u8 = 0x0900_0000 as *mut u8;
global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn not_main() {
  let out_str = b"HyperOS v0.0.1";
  write(out_str);
  loop {}
}

fn write(out_str: &[u8]) {
  for byte in out_str {
    unsafe {
      ptr::write_volatile(UART0, *byte);
    }
  }
}
