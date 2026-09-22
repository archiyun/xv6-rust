use core::hint::spin_loop;
use core::ptr::{read_volatile, write_volatile};

const UART_BASE: usize = 0x1000_0000;
const LSR_OFFSET: usize = 5;
const TX_READY: u8 = 1 << 5;

pub fn put_byte(byte: u8) {
    loop {
        let status = unsafe { read_volatile((UART_BASE + LSR_OFFSET) as *const u8) };

        if status & TX_READY != 0 {
            break;
        }

        spin_loop();
    }

    unsafe {
        write_volatile(UART_BASE as *mut u8, byte);
    }
}

pub fn write_str(text: &str) {
    for byte in text.bytes() {
        if byte == b'\n' {
            put_byte(b'\r');
        }

        put_byte(byte)
    }
}
