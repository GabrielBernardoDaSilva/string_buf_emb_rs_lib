#![no_std]

use core::convert::From;
use core::fmt::Debug;
use core::fmt::Display;
use core::option::Option;
use core::option::Option::None;
use core::option::Option::Some;
pub enum IOError {
    BufferOverflow,
}

impl Debug for IOError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IOError::BufferOverflow => write!(f, "Buffer overflow"),
        }
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IOError::BufferOverflow => write!(f, "Buffer overflow"),
        }
    }
}

pub struct StringBuffer {
    buffer: [u8; 34],
    actual_char: u8,
    cursor_pos: usize,
    actual_buf_display: usize,
}

impl From<&str> for StringBuffer {
    fn from(s: &str) -> Self {
        let mut buffer = StringBuffer::new();
        for c in s.chars() {
            buffer.write_char_to_buffer(c as u8);
            buffer.set_cursor_pos(1);
        }
        buffer
    }
}

impl StringBuffer {
    pub fn new() -> Self {
        Self {
            buffer: ['_' as u8; 34],
            actual_char: 0x20,
            cursor_pos: 0,
            actual_buf_display: 0,
        }
    }

    pub fn select_char(&mut self) {
        let actual_char = self.actual_char + 1;

        if actual_char >= 0x20 && actual_char < 0x7F {
            self.actual_char = actual_char;
        } else {
            self.actual_char = 0x20;
        }
        self.write_char_to_buffer(self.actual_char);
    }

    fn write_char_to_buffer(&mut self, c: u8) {
        self.buffer[self.cursor_pos] = c;
        self.actual_buf_display = self.cursor_pos + 1;
    }

    fn set_cursor_pos(&mut self, x: i32) {
        let x = self.cursor_pos as i32 + x;

        if x > 32 as i32 {
            self.cursor_pos = 0;
        } else if x < 0 {
            self.cursor_pos = 0;
        }

        self.cursor_pos = x as usize;
    }

    fn to_slice(&self, start: Option<usize>, end: Option<usize>) -> &[u8] {
        let start = start.unwrap_or(0);
        let end = end.unwrap_or(self.buffer.len());
        &self.buffer[start..end]
    }

    pub fn to_str(&self) -> &str {
        core::str::from_utf8(self.to_slice(None, Some(self.actual_buf_display))).unwrap_or("Error")
    }

    pub fn set_cursor_x(&mut self, x: u16) -> i8 {
        let x = if x > 999{
            self.actual_char = 0x20;
            1i32
        } else if x < 5 && self.cursor_pos > 0 {
            self.actual_char = self.buffer[self.cursor_pos - 1];
            -1
        } else {
            0
        };
        self.set_cursor_pos(x);
        x as i8
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn get_cursor_pos(&self) -> u8 {
        self.cursor_pos as u8
    }

    pub fn get_display_pos(&self) -> u8 {
        self.actual_buf_display as u8
    }
}
