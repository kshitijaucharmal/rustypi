#[repr(C, align(16))]
pub struct FramebufferRequest {
    width: u32,
    height: u32,
    virtual_width: u32,
    virtual_height: u32,
    pitch: u32,
    depth: u32,
    x_offset: u32,
    y_offset: u32,
    framebuffer_pointer: u32,
    framebuffer_size: u32,
}

pub struct Mailbox;

impl Mailbox {
    pub const MAILBOX_BASE: usize = 0x3F00B880;

    pub fn write(channel: u8, data: u32) {
        let address = Self::MAILBOX_BASE + 0x20;
        unsafe {
            while (core::ptr::read_volatile(Self::MAILBOX_BASE as *const u32) & 0x80000000) != 0 {}
            core::ptr::write_volatile(
                address as *mut u32,
                (data & 0xFFFFFFF0) | (channel as u32 & 0xF),
            );
        }
    }

    pub fn read(channel: u8) -> u32 {
        let address = Self::MAILBOX_BASE;
        unsafe {
            loop {
                while (core::ptr::read_volatile(address as *const u32) & 0x40000000) != 0 {}
                let response = core::ptr::read_volatile((address + 0x00) as *const u32);
                if (response & 0xF) == channel as u32 {
                    return response & 0xFFFFFFF0;
                }
            }
        }
    }
}

pub fn setup_framebuffer(width: u32, height: u32, depth: u32) -> Option<&'static mut [u8]> {
    static mut FB_REQUEST: FramebufferRequest = FramebufferRequest {
        width: 0,
        height: 0,
        virtual_width: 0,
        virtual_height: 0,
        pitch: 0,
        depth: 0,
        x_offset: 0,
        y_offset: 0,
        framebuffer_pointer: 0,
        framebuffer_size: 0,
    };

    unsafe {
        FB_REQUEST.width = width;
        FB_REQUEST.height = height;
        FB_REQUEST.virtual_width = width;
        FB_REQUEST.virtual_height = height;
        FB_REQUEST.depth = depth;
        FB_REQUEST.x_offset = 0;
        FB_REQUEST.y_offset = 0;

        Mailbox::write(1, &FB_REQUEST as *const _ as u32);
        let response = Mailbox::read(1);

        if response == 0 && FB_REQUEST.framebuffer_pointer != 0 {
            let fb_ptr = (FB_REQUEST.framebuffer_pointer & 0x3FFFFFFF) as *mut u8;
            return Some(core::slice::from_raw_parts_mut(
                fb_ptr,
                FB_REQUEST.framebuffer_size as usize,
            ));
        }
    }
    None
}

pub fn draw_pixel(framebuffer: &mut [u8], width: usize, x: usize, y: usize, color: u32) {
    let bytes_per_pixel = 4; // RGBA
    let index = (y * width + x) * bytes_per_pixel;

    framebuffer[index] = (color & 0xFF) as u8; // Red
    framebuffer[index + 1] = ((color >> 8) & 0xFF) as u8; // Green
    framebuffer[index + 2] = ((color >> 16) & 0xFF) as u8; // Blue
    framebuffer[index + 3] = ((color >> 24) & 0xFF) as u8; // Alpha
}
