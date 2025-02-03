#![no_std]
#![no_main]

mod gpio;
use crate::gpio::GPIO;

// Code assembly and PanicModule
use core::arch::asm;
use core::panic::PanicInfo;

// Set Boot point to _start
mod boot {
    use core::arch::global_asm;

    global_asm! {
        ".section .text._start"
    }
}

const LED_PIN: u32 = 47;
const DELAY: u32 = 500_000;

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let width = 800;
    // let height = 600;
    // let depth = 32;
    //
    // if let Some(framebuffer) = setup_framebuffer(width, height, depth) {
    //     for y in 100..200 {
    //         for x in 100..200 {
    //             draw_pixel(framebuffer, width as usize, x, y, 0xFF0000); // Red color
    //         }
    //     }
    // }
    //
    // loop {}

    // Simple blink code -------------------------------------------

    // Turn PIN21 into output
    GPIO::set_output(LED_PIN);

    loop {
        // Turn Pin21 ON
        GPIO::set(LED_PIN);
        delay(DELAY);
        // Turn Pin21 OFF
        GPIO::clear(LED_PIN);
        delay(DELAY);
    }

    // -------------------------------------------------------------
}

fn delay(delay_amt: u32) {
    for _ in 1..delay_amt {
        unsafe {
            asm!("nop");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
