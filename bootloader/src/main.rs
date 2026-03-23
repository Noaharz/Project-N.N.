#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry!(efi_main);

fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let _ = system_table.stdout().reset(false);
    let _ = system_table
        .stdout()
        .write_str("Project N.N. UEFI bootloader\r\n");
    let _ = system_table.stdout().write_str("Hello UEFI\r\n");

    // TODO: initialize UEFI services, load kernel, exit boot services, jump to kernel entry.
    Status::SUCCESS
}
