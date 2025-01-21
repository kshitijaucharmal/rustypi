// 0x 3F20_0008 fsel2 1<<3 turn pin21 into output
// 0x 3F20_001C gpio1_set 1<<21 turns pin21 ON
// 0x 3F20_0028 gpio1_clear 1<<21 turns pin21 OFF

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;
const GPIO_FSEL3: u32 = 0x3F20_000C;
const GPIO_FSEL4: u32 = 0x3F20_0010;
const GPIO_FSEL5: u32 = 0x3F20_0014;

const GPIO_SET0: u32 = 0x3F20_001C;
const GPIO_CLR0: u32 = 0x3F20_0028;

pub struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            3 => GPIO_FSEL3,
            4 => GPIO_FSEL4,
            5 => GPIO_FSEL5,
            _ => panic!("Not a GPIO pin"),
        };

        let mut val: u32 = 0;
        // Make the compiler happy
        let _ = val;

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        // Basically just clear the bits so that bit shift operation is not wrong
        // Steps:
        // Create Mask
        let mut mask: u32 = 0b111;

        // Shift mask to location
        let pin_num = pin % 10;
        mask = mask << (pin_num * 3);

        // AND in the NOT of the mask
        val = val & !(mask);

        // set OUR value
        val |= 1 << pin_num * 3;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn set(pin: u32) {
        let bitpos = pin;

        let mut val: u32 = 0;
        let _ = val;

        unsafe {
            val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
        }
    }
    pub fn clear(pin: u32) {
        let bitpos = pin;

        let mut val: u32 = 0;
        let _ = val;

        unsafe {
            val = core::ptr::read_volatile(GPIO_CLR0 as *mut u32);
        }

        val |= 1 << bitpos;

        unsafe {
            core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val);
        }
    }
}
