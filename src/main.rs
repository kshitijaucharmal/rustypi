#![no_std]
#![no_main]

mod gpio;

use gpio::GPIO;

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    GPIO::set_output(47);

    GPIO::set(47);
    delay();
    GPIO::clear(47);
    delay();

    loop {}
}

fn delay() {
    for _ in 0..1000_000 {
        unsafe {
            asm!("nop");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
