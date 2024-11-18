use std::marker::PhantomData;
use std::ops::Range;

pub type Index = u32;

pub struct Span<T> {
    pub value: T,
    pub range: Range<Index>,
}

pub struct Cursor<'a> {
    start_left: *const u8,
    end_left: *const u8,
    start_right: *const u8,
    end_right: *const u8,
    cursor: *const u8,
    _marker: PhantomData<&'a u8>,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new((left, right): (&'a str, &'a str)) -> Self {
        let start_left = left.as_ptr();
        let start_right = right.as_ptr();

        Cursor {
            start_left,
            end_left: unsafe { start_left.add(left.len()) },
            start_right,
            end_right: unsafe { start_right.add(right.len()) },
            cursor: start_left,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.cursor as usize == self.end_left as usize - 1 {
            if self.start_right == self.end_right {
                None
            } else {
                Some(unsafe { *self.start_right })
            }
        } else if self.cursor != self.end_right {
            Some(unsafe { *self.cursor })
        } else {
            None
        }
    }

    #[inline]
    pub fn advance(&mut self) {
        if self.cursor as usize == self.end_left as usize - 1 {
            self.cursor = self.start_right;
        } else if self.cursor != self.end_right {
            self.cursor = unsafe { self.cursor.add(1) };
        }
    }

    #[inline]
    pub fn skip_whitespace(&mut self) {
        while let Some(byte) = self.peek() {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }
    
    #[inline]
    pub fn index(&self) -> Index {
        if (self.cursor as usize) < self.end_left as usize {
            (self.cursor as usize - self.start_left as usize) as Index
        } else {
            let left_bytes = self.end_left as usize - self.start_left as usize;
            left_bytes as Index + (self.cursor as usize - self.start_right as usize) as Index
        }
    }
}