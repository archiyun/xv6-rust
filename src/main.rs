#![no_std]
#![no_main]

mod uart;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

core::arch::global_asm!(include_str!("entry.S"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart::write_str("xv6-rust is booting\n");
    uart::write_str("UART ready\n");

    loop {
        core::hint::spin_loop();
    }
}
