// #![no_std]
// #![no_main]

// use core::arch::asm;
// use core::ptr;

// // UART
// const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
// //GPIO
// const GPIO_FSEL0: u32 = 0x3F20_0000;
// const GPIO_FSEL1: u32 = 0x3F20_0004;
// const GPIO_FSEL2: u32 = 0x3F20_0008;
// const GPIO_SETO: u32 = 0x3f20_001c;
// const GPIO_CLRO: u32 = 0x3f20_0028;

// // const FRAMEBUFFER_BASE: *mut u32 = 0x3E000000 as *mut u32;

// const MAILBOX_BASE: u32 = 0x3F00B880; // Mailbox base address for RPi3
// const FRAMEBUFFER_CHANNEL: u32 = 1; // Mailbox channel 1 (framebuffer)

// #[repr(align(16))]
// struct FramebufferRequest {
//     buffer: [u32; 26],
// }

// /// Global framebuffer request structure (must be 16-byte aligned)
// static mut FB_REQUEST: FramebufferRequest = FramebufferRequest {
//     buffer: [
//         26 * 4,
//         0, // Buffer size & request code
//         0x00048003,
//         8,
//         8,
//         640,
//         480,
//         640,
//         480, // Set physical & virtual size
//         0x00048005,
//         4,
//         4,
//         32,
//         0, // Set depth (32-bit)
//         0x00048009,
//         4,
//         4,
//         0,
//         0, // Set pixel order (BGR=0, RGB=1)
//         0x00040001,
//         8,
//         8,
//         0,
//         0, // Request framebuffer base address
//         0,
//         0, // End tag
//     ],
// };

// /// Writes to the mailbox
// fn mailbox_write(channel: u32, data: u32) {
//     let mailbox_full = (MAILBOX_BASE + 0x18) as *mut u32; // Mailbox status register
//     let mailbox_write = (MAILBOX_BASE + 0x20) as *mut u32; // Mailbox write register

//     unsafe {
//         while mailbox_full.read_volatile() & 0x80000000 != 0 {} // Wait if full
//         mailbox_write.write_volatile((data & !0xF) | (channel & 0xF));
//     }
// }

// /// Reads from the mailbox
// fn mailbox_read(channel: u32) -> u32 {
//     let mailbox_empty = (MAILBOX_BASE + 0x18) as *mut u32; // Mailbox status register
//     let mailbox_read = (MAILBOX_BASE + 0x00) as *mut u32; // Mailbox read register
//     let mut response: u32;

//     unsafe {
//         loop {
//             while mailbox_empty.read_volatile() & 0x40000000 != 0 {} // Wait if empty
//             response = mailbox_read.read_volatile();
//             if (response & 0xF) == channel {
//                 return response & !0xF;
//             }
//         }
//     }
// }

// /// Initializes the framebuffer
// fn framebuffer_init() -> *mut u32 {
//     unsafe {
//         let fb_addr = &FB_REQUEST.buffer as *const _ as u32;
//         mailbox_write(FRAMEBUFFER_CHANNEL, fb_addr);
//         mailbox_read(FRAMEBUFFER_CHANNEL);

//         let fb_base = FB_REQUEST.buffer[19]; // The GPU writes the framebuffer address here
//         fb_base as *mut u32
//     }
// }

// /// Draws 200 red pixels
// fn draw_red_pixels(fb: *mut u32) {
//     let red = 0x00FF0000; // RGB: (255, 0, 0)
//     unsafe {
//         for i in 0..200 {
//             fb.add(i).write_volatile(red);
//         }
//     }
// }

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

// struct GPIO;

// impl GPIO {
//     // pin_no. mod 10 = GPIO Alternate function select register no
//     pub fn set_output(pin: u32) {
//         let reg = pin / 10;
//         let register = match reg {
//             0 => GPIO_FSEL0,
//             1 => GPIO_FSEL1,
//             2 => GPIO_FSEL2,
//             _ => panic!("Invalid pin no."),
//         };

//         let mut val: u32 = 0;

//         unsafe {
//             val = core::ptr::read_volatile(register as *mut u32);
//         }

//         // create a mask or rather reset all the pin to 000
//         let mut mask: u32 = 0b111;

//         //shift the mask to right location

//         // pinnum = (pin mod 10)*3
//         let pinnum = pin % 10;
//         mask = mask << pinnum * 3;

//         // and the NOT of the mask
//         val = val & !(mask);

//         //set OUR output value
//         val |= 1 << pinnum * 3;

//         unsafe {
//             core::ptr::write_volatile(register as *mut u32, val);
//         }
//     }

//     pub fn set(pin: u32) {
//         let bitpos = pin;

//         let mut val: u32 = 0;

//         unsafe {
//             val = core::ptr::read_volatile(GPIO_SETO as *mut u32);
//         }

//         val |= 1 << bitpos;

//         unsafe {
//             core::ptr::write_volatile(GPIO_SETO as *mut u32, val);
//         }
//     }

//     pub fn clear(pin: u32) {
//         let bitpos = pin;

//         let mut val: u32 = 0;

//         unsafe {
//             val = core::ptr::read_volatile(GPIO_CLRO as *mut u32);
//         }

//         val |= 1 << bitpos;

//         unsafe {
//             core::ptr::write_volatile(GPIO_CLRO as *mut u32, val);
//         }
//     }
// }

// #[link_section = ".text._start"]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     // let msg = b"HI\n";

//     // for &c in msg {
//     //     unsafe {
//     //         ptr::write_volatile(UART0_DR, c as u32);
//     //     }
//     // }

//     // GPIO::set_output(21);

//     // loop{

//     // GPIO::set(21);
//     //for _ in 1..50000{
//     //    unsafe {asm!("nop");}
//     //}
//     //GPIO::clear(21);
//     //for _ in 1..50000{
//     //    unsafe {asm!("nop");}
//     //}

//     //}

//     let fb = framebuffer_init();
//     draw_red_pixels(fb);

//     loop {}
// }

// Dead Code

#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr;

const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
const MAILBOX_BASE: u32 = 0x3F00B880;
const FRAMEBUFFER_CHANNEL: u32 = 1;

#[repr(align(16))]
struct FramebufferRequest {
    buffer: [u32; 21], // Corrected size
}

static mut FB_REQUEST: FramebufferRequest = FramebufferRequest {
    buffer: [
        21 * 4,
        0, // Buffer size & request code
        0x00048003,
        8,
        8,
        640,
        480,
        640,
        480, // Physical & virtual size
        0x00048005,
        4,
        4,
        32,
        0, // Pixel depth
        0x00040001,
        8,
        8,
        0,
        0, // Framebuffer address request
        0,
        0, // End tag
    ],
};

fn mailbox_write(channel: u32, data: u32) {
    let mailbox_full = (MAILBOX_BASE + 0x18) as *mut u32;
    let mailbox_write = (MAILBOX_BASE + 0x20) as *mut u32;

    unsafe {
        while mailbox_full.read_volatile() & 0x80000000 != 0 {} // Wait while full
        mailbox_write.write_volatile((data & !0xF) | (channel & 0xF));
    }
}

fn mailbox_read(channel: u32) -> u32 {
    let mailbox_empty = (MAILBOX_BASE + 0x18) as *mut u32;
    let mailbox_read = (MAILBOX_BASE + 0x00) as *mut u32;

    unsafe {
        loop {
            while mailbox_empty.read_volatile() & 0x40000000 != 0 {} // Wait while empty
            let response = mailbox_read.read_volatile();
            if (response & 0xF) == channel {
                return response & !0xF;
            }
        }
    }
}

fn uart_print(msg: &str) {
    for &byte in msg.as_bytes() {
        unsafe {
            ptr::write_volatile(UART0_DR, byte as u32);
        }
    }
}

fn framebuffer_init() -> Option<*mut u32> {
    unsafe {
        let fb_addr = &FB_REQUEST.buffer as *const _ as u32;
        uart_print("Requesting framebuffer...\n");

        mailbox_write(FRAMEBUFFER_CHANNEL, fb_addr);
        let response = mailbox_read(FRAMEBUFFER_CHANNEL);

        if response != fb_addr {
            uart_print("Mailbox response mismatch: 0x");
            uart_print_hex(response);
            uart_print("\n");
            return None;
        }

        let fb_base = FB_REQUEST.buffer[13]; // Corrected index
        if fb_base == 0 {
            uart_print("Framebuffer allocation failed\n");
            return None;
        }

        uart_print("Framebuffer allocated at 0x");
        uart_print_hex(fb_base);
        uart_print("\n");
        Some(fb_base as *mut u32)
    }
}

fn uart_print_hex(value: u32) {
    let hex_chars = b"0123456789ABCDEF";
    let mut buf = [
        b'0', b'x', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'\n', 0,
    ];

    for i in 0..8 {
        buf[10 - i] = hex_chars[((value >> (i * 4)) & 0xF) as usize];
    }

    uart_print(core::str::from_utf8(&buf).unwrap_or("ERROR\n"));
}

fn draw_green_screen(fb: *mut u32) {
    let green = 0x00FF00; // Green in RGB (hex)
    unsafe {
        for i in 0..640 * 480 {
            fb.add(i).write_volatile(green);
        }
    }
}

fn test_mailbox_communication() {
    const TEST_DATA: u32 = 0x12345678;
    const CHANNEL: u32 = 1;

    mailbox_write(CHANNEL, TEST_DATA);
    let response = mailbox_read(CHANNEL);

    if response == TEST_DATA {
        uart_print("Mailbox Communication Succeeded\n");
    } else {
        uart_print("Mailbox Communication Failed\n");
        uart_print("Expected: 0x");
        uart_print_hex(TEST_DATA);
        uart_print("Received: 0x");
        uart_print_hex(response);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    uart_print("Kernel Panic!\n");
    loop {}
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello, Booting Kernel...\n";
    for &c in msg {
        unsafe {
            ptr::write_volatile(0x3F201000 as *mut u32, c as u32);
        }
    }

    uart_print("Booting kernel...\n"); // DEBUG MESSAGE

    test_mailbox_communication();

    match framebuffer_init() {
        Some(fb) => {
            draw_green_screen(fb);
            uart_print("Green screen drawn!\n"); // DEBUG MESSAGE
        }
        None => {
            uart_print("Framebuffer setup failed\n");
        }
    }

    loop {}
}
