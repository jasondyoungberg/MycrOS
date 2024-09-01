use core::slice;

use bit_field::BitField;
use limine::request::FramebufferRequest;
use spin::{Lazy, Mutex};

use crate::arch::volatile::Volatile;

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

const FONT_DATA: &[u8] = include_bytes!("font.bin");
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 16;

pub static FRAMEBUFFER: Lazy<Mutex<Framebuffer>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_REQUEST
        .get_response()
        .unwrap()
        .framebuffers()
        .next()
        .unwrap();

    assert_eq!(framebuffer.bpp(), 32);

    let width = framebuffer.width().try_into().unwrap();
    let height = framebuffer.height().try_into().unwrap();
    let stride = (framebuffer.pitch() / 4).try_into().unwrap();

    let buffer_ptr = framebuffer.addr().cast();
    let buffer_size = height * stride;
    let buffer = unsafe { slice::from_raw_parts_mut(buffer_ptr, buffer_size) };

    Mutex::new(Framebuffer {
        buffer,
        width,
        height,
        stride,
    })
});

pub struct Framebuffer {
    buffer: &'static mut [Volatile<u32>],
    width: usize,
    height: usize,
    stride: usize,
}

impl Framebuffer {
    pub fn draw_pix(&mut self, color: u32, pos: (usize, usize)) {
        self.buffer[pos.0 + pos.1 * self.stride].write(color);
    }

    pub fn draw_char(&mut self, c: char, pos: (usize, usize)) {
        let (char_x, char_y) = pos;

        let char_id = match c {
            ' '..='~' => c as usize - ' ' as usize,
            _ => 95,
        };

        let bitmap = &FONT_DATA[FONT_HEIGHT * char_id..FONT_HEIGHT * (char_id + 1)];

        let base_x = char_x * FONT_WIDTH;
        let base_y = char_y * FONT_HEIGHT;

        for (offset_y, row) in bitmap.iter().enumerate() {
            for offset_x in 0..FONT_WIDTH {
                let color = if row.get_bit(FONT_WIDTH - offset_x - 1) {
                    0xFFFFFFFF
                } else {
                    0
                };
                let pix_x = base_x + offset_x;
                let pix_y = base_y + offset_y;

                self.draw_pix(color, (pix_x, pix_y));
            }
        }
    }
}
