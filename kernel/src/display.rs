use core::fmt::Write;

use bit_field::BitField;
use spin::{Lazy, Mutex};

use crate::boot::FRAMEBUFFER_RESPONSE;

const FONT_DATA: &[u8] = include_bytes!("font.bin");
const FONT_WIDTH: u64 = 8;
const FONT_HEIGHT: u64 = 16;

pub static DISPLAY: Lazy<Mutex<Display>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_RESPONSE
        .framebuffers()
        .next()
        .expect("There should be at least one framebuffer");

    let ptr = framebuffer.addr();
    let size = framebuffer.pitch() * framebuffer.height();

    // Safety: `ptr` and `size` are provided and validated by limine
    let buffer = unsafe { core::slice::from_raw_parts_mut(ptr, size as usize) };

    assert_eq!(
        framebuffer.bpp() % 8,
        0,
        "Bits per pixel should be a multiple of 8"
    );

    let display = Display {
        buffer,
        total_bytes: framebuffer.pitch() * framebuffer.height(),
        bytes_per_line: framebuffer.pitch(),
        bytes_per_pixel: u64::from(framebuffer.bpp()) / 8,
        width: framebuffer.width(),
        height: framebuffer.height(),
        cursor_x: 0,
        cursor_y: 0,
    };

    Mutex::new(display)
});

pub struct Display {
    buffer: &'static mut [u8],

    total_bytes: u64,
    bytes_per_line: u64,
    bytes_per_pixel: u64,

    width: u64,
    height: u64,

    cursor_x: u64,
    cursor_y: u64,
}

impl Display {
    fn set_pixel(&mut self, x: u64, y: u64, state: bool) {
        let main_offset = y * self.bytes_per_line + x * self.bytes_per_pixel;
        let color = if state { 0xFF } else { 0x00 };

        for color_offset in 0..self.bytes_per_pixel {
            self.buffer[(main_offset + color_offset) as usize] = color;
        }
    }

    fn put_char(&mut self, c: char) {
        let char_x = self.cursor_x;
        let char_y = self.cursor_y;

        let char_index = match c {
            '\n' => {
                self.newline();
                return;
            }
            ' '..='~' => c as u64 - 32,
            _ => 95,
        };

        let font_offset = char_index * FONT_HEIGHT;
        let font_data = &FONT_DATA[font_offset as usize..(font_offset + FONT_HEIGHT) as usize];

        for sub_y in 0..FONT_HEIGHT {
            for sub_x in 0..FONT_WIDTH {
                self.set_pixel(
                    char_x + sub_x,
                    char_y + sub_y,
                    font_data[sub_y as usize].get_bit((FONT_WIDTH - sub_x - 1) as usize),
                );
            }
        }

        self.cursor_x += FONT_WIDTH;

        if self.cursor_x >= self.width {
            self.newline();
        }
    }

    fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += FONT_HEIGHT;
        if self.cursor_y >= self.height {
            self.cursor_y -= FONT_HEIGHT;
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        let offset = FONT_HEIGHT * self.bytes_per_line;

        self.buffer.copy_within(offset as usize.., 0);

        self.buffer[(self.total_bytes - offset) as usize..].fill(0);
    }
}

impl Write for Display {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.put_char(c);
        }
        Ok(())
    }
}
