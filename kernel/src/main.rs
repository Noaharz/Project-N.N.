#![no_std]
#![no_main]

mod arch;
mod console;
mod mem;

use core::panic::PanicInfo;
use boot_protocol::{FramebufferFormat, Handoff};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(_handoff: *const Handoff) -> ! {
    // Minimal entry point. Bootloader will jump here.
    arch::init();
    console::init();

    console::write_str("Kernel reached _start\\n");

    // Touch the handoff to avoid unused warnings later when we start using it.
    unsafe {
        let _fb = (*_handoff).framebuffer.format as u32;
        let _ = _fb == FramebufferFormat::Unknown as u32;
    }

    loop {}
}
