use core::ptr::NonNull;

use bit_field::BitField;
use limine::request::FramebufferRequest;
use spin::{Lazy, Mutex};
use volatile::{access::WriteOnly, VolatileRef};

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

const FONT_DATA: &[u8] = include_bytes!("font.bin");
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 16;

pub struct Framebuffer {
    buffer: VolatileRef<'static, u32, WriteOnly>,
    width: usize,
    height: usize,
    stride: usize,
}

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

    let addr = NonNull::new(framebuffer.addr().cast()).unwrap();
    let buffer = unsafe { VolatileRef::new_restricted(WriteOnly, addr) };

    Mutex::new(Framebuffer {
        buffer,
        width,
        height,
        stride,
    })
});

impl Framebuffer {
    pub fn draw_pix(&mut self, color: u32, pos: (usize, usize)) {
        let index = pos.0 + pos.1 * self.stride;
        assert!(pos.0 < self.width);
        assert!(pos.1 < self.height);
        unsafe { self.buffer.as_mut_ptr().map(|x| x.add(index)) }.write(color)
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
