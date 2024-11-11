#![no_std]
#![no_main]
#![no_implicit_prelude]

use core::arch::asm;
use core::panic::PanicInfo;

fn ft_putchar(c: u8) {
	asm!(
		"mov rax, 1",        // syscall: sys_write
		"mov rdi, 1",        // file descriptor: stdout
		"mov rsi, {0}",      // buffer: address of the character
		"mov rdx, 1",        // buffer length: 1
		"syscall",           // invoke syscall
		in(reg) &c,          // input: address of the character
		options(nostack, preserves_flags)
	);
}

fn ft_exit(code: u8) -> ! {
	asm!(
		"mov rax, 60",       // syscall: sys_exit
		"mov rdi, {0}",      // exit code
		"syscall",           // invoke syscall
		in(reg) code,        // input: exit code
		options(noreturn, nostack, preserves_flags)
	);
	loop {} // infinite loop to ensure the function never returns
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}