#![no_std]
#![no_main]

use core::ptr;
use core::arch::asm;

// UART 
const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
//GPIO
const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;
const GPIO_SETO: u32 = 0x3f20_001c;
const GPIO_CLRO: u32 = 0x3f20_0028;


struct GPIO;

impl GPIO{
    // pin_no. mod 10 = GPIO Alternate function select register no
    pub fn set_output(pin: u32) {
        let reg = pin/10;
        let register = match reg{
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Invalid pin no."),
        };

        let mut val: u32 =0;
        
        unsafe{
            val = core::ptr::read_volatile(register as *mut u32);
        }

        // create a mask or rather reset all the pin to 000 
        let mut mask: u32 = 0b111;
        
        //shift the mask to right location
        
        // pinnum = (pin mod 10)*3
        let pinnum = pin%10;
        mask = mask<<pinnum*3;
        
        // and the NOT of the mask
        val = val & !(mask); 
        
        //set OUR output value
        val |= 1<<pinnum*3;
        
        unsafe{
            core::ptr::write_volatile(register as *mut u32,val);
        }


    }

    pub fn set(pin: u32) {
        let bitpos = pin;

        let mut val: u32 = 0;

        unsafe{
            val = core::ptr::read_volatile(GPIO_SETO as *mut u32);
        }

        val |= 1<<bitpos;

        unsafe{
            core::ptr::write_volatile(GPIO_SETO as *mut u32,val);
        }

    }

    pub fn clear(pin: u32) {
        let bitpos = pin;

        let mut val: u32 = 0;

        unsafe{
            val = core::ptr::read_volatile(GPIO_CLRO as *mut u32);
        }

        val |= 1<<bitpos;

        unsafe{
            core::ptr::write_volatile(GPIO_CLRO as *mut u32,val);
        }

    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    let msg = b"HI\n";

    for &c in msg {
        unsafe {
            ptr::write_volatile(UART0_DR, c as u32);
        }
    }

    GPIO::set_output(21);

    loop{

        GPIO::set(21);
        // for _ in 1..50000{
        //    unsafe {asm!("nop");}
        // }
        GPIO::clear(21);
        // for _ in 1..50000{
        //    unsafe {asm!("nop");}
        // }

    }


    loop {

    }
}


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ^^^^^^^^^^^^^For GPIO ^^^^^^^^^^^^^^^^^^^ <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> For Frame Buffer <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// #![no_std]
// #![no_main]

// use core::ptr;

// // UART (For Raspberry Pi 3, these addresses are generally correct)
// const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;

// const FRAMEBUFFER_WIDTH: u32 = 800;
// const FRAMEBUFFER_HEIGHT: u32 = 600;
// const FRAMEBUFFER_ADDRESS: *mut u32 = 0x3F000000 as *mut u32; // This *might* need adjustment

// pub fn draw_pixel(x: u32, y: u32, color: u32) {
//     unsafe {
//         if x < FRAMEBUFFER_WIDTH && y < FRAMEBUFFER_HEIGHT
//         {  
//             let offset = (y * FRAMEBUFFER_WIDTH) + x;
//             FRAMEBUFFER_ADDRESS.add(offset as usize).write_volatile(color);
//             uart_print("Printing a red pixel\n");
//         }
//     }
// }

// pub fn uart_print(s: &str) {
//     for &c in s.as_bytes() {
//         unsafe {
//             ptr::write_volatile(UART0_DR, c as u32);
//         }
//     }
// }

// #[link_section = ".text._start"]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     uart_print("Booting...\n");

//     for y in 0..10 {
//         for x in 0..20 {
//             draw_pixel(x, y, 0xFF0000); // Red
//         }
//     }
//     uart_print("Finished Drawing\n");

//     loop {}
// }

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

