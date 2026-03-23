#[cfg(target_arch = "x86_64")]
use core::arch::asm;

const COM1: u16 = 0x3F8;

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, nomem));
}

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", in("dx") port, out("al") value, options(nostack, nomem));
    value
}

pub fn init() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        outb(COM1 + 1, 0x00); // Disable interrupts
        outb(COM1 + 3, 0x80); // Enable DLAB
        outb(COM1 + 0, 0x03); // Divisor low (38400 baud)
        outb(COM1 + 1, 0x00); // Divisor high
        outb(COM1 + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(COM1 + 2, 0xC7); // Enable FIFO, clear, 14-byte threshold
        outb(COM1 + 4, 0x0B); // IRQs enabled, RTS/DSR set
    }
}

pub fn write_byte(byte: u8) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        while (inb(COM1 + 5) & 0x20) == 0 {}
        outb(COM1, byte);
    }
}

pub fn write_str(s: &str) {
    for b in s.bytes() {
        match b {
            b'\n' => {
                write_byte(b'\r');
                write_byte(b'\n');
            }
            _ => write_byte(b),
        }
    }
}
