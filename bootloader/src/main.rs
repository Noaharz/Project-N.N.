#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn efi_main(_image_handle: usize, _system_table: usize) -> usize {
    // TODO: initialize UEFI services, load kernel, exit boot services, jump to kernel entry.
    0
}
