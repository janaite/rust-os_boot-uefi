#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "efiapi" fn efi_main() { 
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
