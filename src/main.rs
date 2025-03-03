// #![no_std]
// #![no_main]
// // #![no_start] // This is the important one!
// use core::ptr;
// use core::arch::asm;

// // UART 
// const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
// //GPIO
// const GPIO_FSEL0: u32 = 0x3F20_0000;
// const GPIO_FSEL1: u32 = 0x3F20_0004;
// const GPIO_FSEL2: u32 = 0x3F20_0008;
// const GPIO_SETO: u32 = 0x3f20_001c;
// const GPIO_CLRO: u32 = 0x3f20_0028;


// struct GPIO;

// impl GPIO{
//     // pin_no. mod 10 = GPIO Alternate function select register no
//     pub fn set_output(pin: u32) {
//         let reg = pin/10;
//         let register = match reg{
//             0 => GPIO_FSEL0,
//             1 => GPIO_FSEL1,
//             2 => GPIO_FSEL2,
//             _ => panic!("Invalid pin no."),
//         };

//         let mut val: u32 =0;
        
//         unsafe{
//             val = core::ptr::read_volatile(register as *mut u32);
//         }

//         // create a mask or rather reset all the pin to 000 
//         let mut mask: u32 = 0b111;
        
//         //shift the mask to right location
        
//         // pinnum = (pin mod 10)*3
//         let pinnum = pin%10;
//         mask = mask<<pinnum*3;
        
//         // and the NOT of the mask
//         val = val & !(mask); 
        
//         //set OUR output value
//         val |= 1<<pinnum*3;
        
//         unsafe{
//             core::ptr::write_volatile(register as *mut u32,val);
//         }


//     }

//     pub fn set(pin: u32) {
//         let bitpos = pin;

//         let mut val: u32 = 0;

//         unsafe{
//             val = core::ptr::read_volatile(GPIO_SETO as *mut u32);
//         }

//         val |= 1<<bitpos;

//         unsafe{
//             core::ptr::write_volatile(GPIO_SETO as *mut u32,val);
//         }

//     }

//     pub fn clear(pin: u32) {
//         let bitpos = pin;

//         let mut val: u32 = 0;

//         unsafe{
//             val = core::ptr::read_volatile(GPIO_CLRO as *mut u32);
//         }

//         val |= 1<<bitpos;

//         unsafe{
//             core::ptr::write_volatile(GPIO_CLRO as *mut u32,val);
//         }

//     }
// }

// #[link_section = ".text._start"]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
    
//     let msg = b"HI\n";

//     for &c in msg {
//         unsafe {
//             ptr::write_volatile(UART0_DR, c as u32);
//         }
//     }

//     GPIO::set_output(21);

//     loop{

//         GPIO::set(21);
//         // for _ in 1..50000{
//         //    unsafe {asm!("nop");}
//         // }
//         GPIO::clear(21);
//         // for _ in 1..50000{
//         //    unsafe {asm!("nop");}
//         // }

//     }
// }


// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ^^^^^^^^^^^^^For GPIO ^^^^^^^^^^^^^^^^^^^ <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> For Frame Buffer <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// #![no_std]
// #![no_main]
// #![no_start] // This is the important one!

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

//     loop {
//         for y in 0..10 {
//             for x in 0..20 {
//                 draw_pixel(x, y, 0xFF0000); // Red
//             }
//         }
//     }
// }

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }


// >>>>>>>>>>>>>>>>>>>>>>>>> UART WITH INPUT clock 250Mhz<<<<<<<<<<<<<<<<<<<<<<<

// #![no_std]
// #![no_main]

// use core::ptr;
// use core::arch::asm;
// use core::fmt::Write;

// // UART0
// const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
// const UART0_FR: *mut u32 = 0x3F201018 as *mut u32; // Flag register
// const UART0_IBRD: *mut u32 = 0x3F201024 as *mut u32; // Integer Baud Rate Divisor
// const UART0_FBRD: *mut u32 = 0x3F201028 as *mut u32; // Fractional Baud Rate Divisor
// const UART0_LCRH: *mut u32 = 0x3F20102C as *mut u32; // Line Control Register High
// const UART0_CR: *mut u32 = 0x3F201030 as *mut u32; // Control Register

// // UART functions
// fn uart_init() {
//     unsafe {
//         // Disable UART
//         ptr::write_volatile(UART0_CR, 0x00000000);

//         // Set baud rate to 115200 (adjust if needed)
//         // Assuming system clock frequency is 250MHz. Adjust if needed.
//         let baud_rate_divisor = 270; // Calculated for 115200 baud. Adjust if needed.
//         ptr::write_volatile(UART0_IBRD, baud_rate_divisor & 0x0000FFFF); // Integer part
//         ptr::write_volatile(UART0_FBRD, (baud_rate_divisor >> 16) & 0x0000003F); // Fractional part

//         // Set 8-bit data, no parity, 1 stop bit
//         ptr::write_volatile(UART0_LCRH, 0x00000060);

//         // Enable FIFO, enable transmit and receive
//         ptr::write_volatile(UART0_CR, 0x00000301);
//     }
// }

// fn uart_send(c: u8) {
//     unsafe {
//         ptr::write_volatile(UART0_DR, c as u32);
//     }
// }

// fn uart_send_string(s: &[u8]) {
//     for &byte in s {
//         uart_send(byte);
//     }
// }

// fn uart_receive() -> u8 {
//     unsafe {
//         loop {
//             let fr = ptr::read_volatile(UART0_FR);
//             if fr & (1 << 4) != 0 {  // Check RX FIFO not empty flag
//                 asm!("nop"); // Or a short delay if needed
//             } else {
//                 break; // Data available, exit the loop
//             }
//         }
//         ptr::read_volatile(UART0_DR) as u8
//     }
// }

// struct Uart; // Empty struct for implementing Write trait

// impl Write for Uart {
//     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//         for byte in s.as_bytes() {
//             uart_send(*byte); // Dereference byte to get u8 value
//         }
//         Ok(())
//     }
// }

// #[link_section = ".text._start"]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     uart_init(); // Initialize UART

//     let mut uart = Uart; // Create an instance of the Uart struct

//     // Use the write! macro on the Uart instance
//     let _ = write!(&mut uart, "Hello, world! Type something:\n"); 

//     loop {
//         let input_byte = uart_receive();

//         let _ = write!(&mut uart, "You typed: "); 
//         uart_send(input_byte); // Send the actual typed byte
//         let _ = write!(&mut uart, "\n");  
//     }
// }

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

// >>>>>>>>>>>>>>>>>>>>>>>>> UART WITH INPUT clock 48Mhz<<<<<<<<<<<<<<<<<<<<<<<

// #![no_std]
// #![no_main]

// use core::ptr;
// use core::arch::asm;
// use core::fmt::Write;

// // UART0
// const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
// const UART0_FR: *mut u32 = 0x3F201018 as *mut u32; // Flag register
// const UART0_IBRD: *mut u32 = 0x3F201024 as *mut u32; // Integer Baud Rate Divisor
// const UART0_FBRD: *mut u32 = 0x3F201028 as *mut u32; // Fractional Baud Rate Divisor
// const UART0_LCRH: *mut u32 = 0x3F20102C as *mut u32; // Line Control Register High
// const UART0_CR: *mut u32 = 0x3F201030 as *mut u32; // Control Register

// // UART functions
// fn uart_init() {
//     unsafe {
//       // Disable UART before configuration
//       ptr::write_volatile(UART0_CR, 0x00000000);

//       // Set baud rate to 115200 (assuming 48MHz clock)
//       let baud_div = 48_000_000 / (16 * 115_200);
//       ptr::write_volatile(UART0_IBRD, baud_div & 0xFFFF);
//       ptr::write_volatile(UART0_FBRD, ((48_000_000 % (16 * 115_200)) * 64 + 115_200 / 2) / 115_200);

//       // Set 8-bit data, no parity, 1 stop bit
//       ptr::write_volatile(UART0_LCRH, 0x00000060);

//       // Enable TX, RX, and UART
//       ptr::write_volatile(UART0_CR, 0x0301);
//     }
// }

// fn uart_send(c: u8) {
//     unsafe {
//         ptr::write_volatile(UART0_DR, c as u32);
//     }
// }

// fn uart_send_string(s: &[u8]) {
//     for &byte in s {
//         uart_send(byte);
//     }
// }

// fn uart_receive() -> u8 {
//     unsafe {
//         loop {
//             let fr = ptr::read_volatile(UART0_FR);
//             if fr & (1 << 4) != 0 {  // Check RX FIFO not empty flag
//                 asm!("nop"); // Or a short delay if needed
//             } else {
//                 break; // Data available, exit the loop
//             }
//         }
//         ptr::read_volatile(UART0_DR) as u8
//     }
// }

// struct Uart; // Empty struct for implementing Write trait

// impl Write for Uart {
//     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//         for byte in s.as_bytes() {
//             uart_send(*byte); // Dereference byte to get u8 value
//         }
//         Ok(())
//     }
// }

// #[link_section = ".text._start"]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     uart_init(); // Initialize UART

//     let mut uart = Uart; // Create an instance of the Uart struct

//     // Use the write! macro on the Uart instance
//     let _ = write!(&mut uart, "Hello, world! Type something:\n"); 

//     loop {
//         let input_byte = uart_receive();

//         let _ = write!(&mut uart, "You typed: "); 
//         uart_send(input_byte); // Send the actual typed byte
//         let _ = write!(&mut uart, "\n");  
//     }
// }

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> UART(48Mhz with Buffer for string input)<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<


#![no_std]
#![no_main]

use core::ptr;
use core::arch::asm;
use core::fmt::Write;

// UART0
const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
const UART0_FR: *mut u32 = 0x3F201018 as *mut u32; // Flag register
const UART0_IBRD: *mut u32 = 0x3F201024 as *mut u32; // Integer Baud Rate Divisor
const UART0_FBRD: *mut u32 = 0x3F201028 as *mut u32; // Fractional Baud Rate Divisor
const UART0_LCRH: *mut u32 = 0x3F20102C as *mut u32; // Line Control Register High
const UART0_CR: *mut u32 = 0x3F201030 as *mut u32; // Control Register

// Buffer for storing input
const BUFFER_SIZE: usize = 128;
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut BUFFER_INDEX: usize = 0;


fn print_krhos_logo() {     
    uart_send_string(b"   ________________________________________________________  \n");
    uart_send_string(b"  /________________________________________________________| \n");
    uart_send_string(b" | ##    ##  ######    ##     ##     ## ##       #######   | \n");
    uart_send_string(b" | ##   ##   ##   ##   ##     ##  ##       ##  ##          | \n");
    uart_send_string(b" | ##  ##    ##   ##   ##     ##  ##       ##  ##          | \n");
    uart_send_string(b" | ## #      #####     ## ### ##  ##       ##     ###      | \n");
    uart_send_string(b" | ##  ##    ##   ##   ##     ##  ##       ##        ###   | \n");
    uart_send_string(b" | ##   ##   ##    ##  ##     ##  ##       ##          ##  | \n");
    uart_send_string(b" | ##    ##  ##     ## ##     ##     ## ##      ########   | \n");
    uart_send_string(b" |_________________________________________________________| \n");
    uart_send_string(b" |________________________________________________________/  \n");
    uart_send_string(b"     K         R          H           O            S         \n");
    uart_send_string(b"------------------------v 0.1.0----------------------------- \n");
}


// UART functions
fn uart_init() {
    unsafe {
      // Disable UART before configuration
      ptr::write_volatile(UART0_CR, 0x00000000);

      // Set baud rate to 115200 (assuming 48MHz clock)
      let baud_div = 48_000_000 / (16 * 115_200);
      ptr::write_volatile(UART0_IBRD, baud_div & 0xFFFF);
      ptr::write_volatile(UART0_FBRD, ((48_000_000 % (16 * 115_200)) * 64 + 115_200 / 2) / 115_200);

      // Set 8-bit data, no parity, 1 stop bit
      ptr::write_volatile(UART0_LCRH, 0x00000060);

      // Enable TX, RX, and UART
      ptr::write_volatile(UART0_CR, 0x0301);
    }
}

fn uart_send(c: u8) {
    unsafe {
        ptr::write_volatile(UART0_DR, c as u32);
    }
}

fn uart_send_string(s: &[u8]) {
    for &byte in s {
        uart_send(byte);
    }
}

fn uart_receive() -> u8 {
    unsafe {
        loop {
            let fr = ptr::read_volatile(UART0_FR);
            if fr & (1 << 4) != 0 {  // Check RX FIFO not empty flag
                asm!("nop"); // Or a short delay if needed
            } else {
                break; // Data available, exit the loop
            }
        }
        ptr::read_volatile(UART0_DR) as u8
    }
}

struct Uart; // Empty struct for implementing Write trait

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.as_bytes() {
            uart_send(*byte); // Dereference byte to get u8 value
        }
        Ok(())
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    uart_init(); // Initialize UART

    let mut uart = Uart; // Create an instance of the Uart struct

    print_krhos_logo();
    
    // Use the write! macro on the Uart instance
    let _ = write!(&mut uart, "Hello, world! Type something\n"); 

    
    loop {
        let input_byte = uart_receive();
        
        unsafe {
            if BUFFER_INDEX < BUFFER_SIZE - 1 {
                BUFFER[BUFFER_INDEX] = input_byte;
                BUFFER_INDEX += 1;
                uart_send(input_byte);
            }
            if input_byte == 0x7F{
                if BUFFER_INDEX > 1 {
                    BUFFER_INDEX = BUFFER_INDEX -2; // Remove last character
                    uart_send(0x08); // Move cursor back
                    uart_send(b' '); // Erase character on screen
                    uart_send(0x08); // Move cursor back again
                }
            }
            if input_byte == b'\r' {
                BUFFER[BUFFER_INDEX] = 0; // Null-terminate for safety
                let _ = write!(&mut uart, "\nYou typed: "); 
                uart_send_string(&BUFFER[..BUFFER_INDEX]);
                let _ = write!(&mut uart, "\n");  
                BUFFER_INDEX = 0; // Reset buffer
            }
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
