use std::fmt::{Debug, Formatter, Pointer};
use crate::eval::bytes::Index;
use std::mem::{transmute, MaybeUninit};
use std::ops;
use std::ops::Range;

pub struct TextBox {
    buffer: Box<[MaybeUninit<u8>]>,
    gap_start: usize,
    gap_end: usize,
    
    /// The number of chars left from the cursor.
    chars_left_from_cursor: usize,
    
    /// The number of chars right from the cursor.
    chars_right_from_cursor: usize,
}

impl Debug for TextBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (left, right) = self.parts();
        
        f.debug_tuple("TextBox")
            .field(&left)
            .field(&right)
            .finish()
    }
}

impl TextBox {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    pub fn clear(&mut self) {
        self.gap_start = 0;
        self.gap_end = self.buffer.len();
        self.chars_left_from_cursor = 0;
        self.chars_right_from_cursor = 0;
    }
    
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Box::new_uninit_slice(capacity),
            gap_start: 0,
            gap_end: capacity,
            chars_left_from_cursor: 0,
            chars_right_from_cursor: 0,
        }
    }

    #[inline]
    pub fn move_cursor_n_chars_left(&mut self, n: usize) {
        let bytes_to_move = if n >= self.chars_left_from_cursor {
            self.chars_right_from_cursor += self.chars_left_from_cursor;
            self.chars_left_from_cursor = 0;
            self.gap_start
        } else {
            self.chars_left_from_cursor -= n;
            self.chars_right_from_cursor += n;

            let mut chars_to_pass = n;
            let mut bytes_to_move = 1;

            while chars_to_pass > 0 {
                while unsafe { self.buffer[self.gap_start - bytes_to_move].assume_init() } & 0b1100_0000 == 0b1000_0000 {
                    bytes_to_move += 1;
                }

                chars_to_pass -= 1;
            }

            bytes_to_move
        };
        
        let new_gap_start = self.gap_start - bytes_to_move;
        let new_gap_end = self.gap_end - bytes_to_move;

        self.buffer.copy_within(new_gap_start..self.gap_start, new_gap_end);

        self.gap_start = new_gap_start;
        self.gap_end = new_gap_end;
    }
    
    pub fn move_cursor_n_chars_right(&mut self, n: usize) {
        let bytes_to_move = if n >= self.chars_right_from_cursor {
            self.chars_left_from_cursor += self.chars_right_from_cursor;
            self.chars_right_from_cursor = 0;
            self.buffer.len() - self.gap_end
        } else {
            self.chars_left_from_cursor += n;
            self.chars_right_from_cursor -= n;

            let mut chars_to_pass = n;
            let mut bytes_to_move = 1;

            while chars_to_pass > 0 {
                while unsafe { self.buffer[self.gap_end + bytes_to_move].assume_init() } & 0b1100_0000 == 0b1000_0000 {
                    bytes_to_move += 1;
                }
                
                bytes_to_move += 1;
                chars_to_pass -= 1;
            }

            bytes_to_move - 1
        };
        
        let new_gap_start = self.gap_start + bytes_to_move;
        let new_gap_end = self.gap_end + bytes_to_move;

        self.buffer.copy_within(self.gap_end..new_gap_end, self.gap_start);

        self.gap_start = new_gap_start;
        self.gap_end = new_gap_end;
    }

    #[inline]
    pub fn insert_char(&mut self, c: char) {
        let char_utf8_len = c.len_utf8();

        if char_utf8_len > self.gap_size() {
            todo!("Resize buffer");
        }

        let mut encoded = [0; 4];
        c.encode_utf8(&mut encoded);

        let gap_start = self.gap_start;

        unsafe {
            self.buffer[gap_start..gap_start + char_utf8_len]
                .copy_from_slice(transmute(&encoded[..char_utf8_len]));
        }

        self.gap_start += char_utf8_len;

        self.chars_left_from_cursor += 1;
    }

    #[inline]
    fn gap_size(&self) -> usize {
        self.gap_end - self.gap_start
    }

    #[inline]
    pub fn parts(&self) -> (&str, &str) {
        unsafe {
            // This is safe because the gap is the only thing uninitialized.
            transmute((&self.buffer[..self.gap_start], &self.buffer[self.gap_end..]))
        }
    }

    #[inline]
    pub fn remove_char_left(&mut self) {
        if self.chars_left_from_cursor == 0 {
            return;
        }

        let mut char_utf8_len = 1;

        while unsafe { self.buffer[self.gap_start - char_utf8_len].assume_init() } & 0b1100_0000 == 0b1000_0000 {
            char_utf8_len += 1;
        }

        self.gap_start -= char_utf8_len;
        self.chars_left_from_cursor -= 1;
    }

    #[inline]
    pub fn chars_left_from_cursor(&self) -> usize {
        self.chars_left_from_cursor
    }
    
    #[inline]
    pub fn chars_right_from_cursor(&self) -> usize {
        self.chars_right_from_cursor
    }
    
    #[inline]
    pub fn gap_start(&self) -> usize {
        self.gap_start
    }
    
    #[inline]
    pub fn gap_end(&self) -> usize {
        self.gap_end
    }
    
    pub fn range(&self, Range { start, end }: Range<Index>) -> (&str, &str) {
        if (start as usize) < self.gap_start && end as usize >= self.gap_start {
            let (left, right) = self.parts();
            (
                &left[start as usize..self.gap_start],
                &right[0..end as usize - self.gap_start]
            )
        } else if start < self.gap_start as Index {
            (unsafe {
                transmute(&self.buffer[start as usize..end as usize])
            }, "")
        } else {
            let gap_size = self.gap_size();
            
            ("", unsafe {
                transmute(&self.buffer[start as usize + gap_size..end as usize + gap_size])
            })
        }
    }
}

impl ops::Index<Index> for TextBox {
    type Output = u8;

    fn index(&self, index: Index) -> &Self::Output {
        if index < self.gap_start as Index {
            unsafe {
                self.buffer[index as usize].assume_init_ref()
            }
        } else {
            unsafe {
                self.buffer[(index + self.gap_end as Index - self.gap_start as Index) as usize].assume_init_ref()
            }
        }
    }
}

impl ops::IndexMut<Index> for TextBox {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        if index < self.gap_start as Index {
            unsafe {
                self.buffer[index as usize].assume_init_mut()
            }
        } else {
            unsafe {
                self.buffer[(index + self.gap_end as Index - self.gap_start as Index) as usize].assume_init_mut()
            }
        }
    }
}