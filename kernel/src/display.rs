use core::{fmt::Write, ptr::NonNull};

use bit_field::BitField;
use spin::{Lazy, Mutex};
use volatile::{access::WriteOnly, VolatileRef};

use crate::boot::FRAMEBUFFER_RESPONSE;

const FONT_DATA: &[u8] = include_bytes!("font.bin");
const FONT_WIDTH: u64 = 8;
const FONT_HEIGHT: u64 = 16;

pub static DISPLAY: Lazy<Mutex<Display>> = Lazy::new(|| {
    let framebuffer = FRAMEBUFFER_RESPONSE
        .framebuffers()
        .next()
        .expect("There should be at least one framebuffer");

    let addr = NonNull::new(framebuffer.addr()).expect("Framebuffer address should not be null");
    let buffer = unsafe { VolatileRef::new(addr) }.write_only();

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
    buffer: VolatileRef<'static, u8, WriteOnly>,

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
            let total_offset = main_offset + color_offset;
            assert!(total_offset < self.total_bytes, "Offset is out of bounds");
            let ptr = self.buffer.as_mut_ptr();
            // Safety: `offset` is within the bounds of the buffer.
            let f = |x: NonNull<u8>| unsafe { x.add(total_offset as usize) };
            // Safety: `buffer` is a pointer from limine
            // and `f` adds a bound-checked offset to it.
            unsafe { ptr.map(f) }.write(color);
        }
    }

    fn put_char(&mut self, c: char) {
        let char_x = self.cursor_x;
        let char_y = self.cursor_y;
        self.cursor_x += FONT_WIDTH;

        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += FONT_HEIGHT;
        }

        let char_index = match c {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += FONT_HEIGHT;
                return;
            }
            ' '..='~' => c as u64 - 31,
            _ => 0,
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
