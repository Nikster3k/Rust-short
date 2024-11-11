// main.rs

#![no_std]
#![no_main]
#![no_implicit_prelude]

extern crate core;

use core::panic::PanicInfo;

extern "C" {
    fn ft_putchar(c: u8);
    fn ft_exit(code: u8) -> !;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        ft_putchar(b'4');
        ft_putchar(b'2');
        ft_putchar(b'\n');
        ft_exit(0);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}