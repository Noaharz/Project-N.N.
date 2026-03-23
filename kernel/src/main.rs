#![no_std]
#![no_main]

mod arch;
mod console;
mod mem;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Minimal entry point. Bootloader will jump here.
    arch::init();
    console::init();

    // TODO: replace with real output once VGA/serial are wired.
    loop {}
}
